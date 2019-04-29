#![forbid(unsafe_code)]

use parser_combinators::prelude::*;

use parser_combinators::filter::FilterError;
use parser_combinators::repeat::FoundZero;

use either::Either;

use std::collections::HashMap;

#[derive(Debug)]
struct EmptyInput;

fn any_char() -> impl for<'a> Parser<&'a str, Output = char, Error = EmptyInput> {
    (|s: &mut &str| {
        let c = s.chars().next().ok_or(EmptyInput)?;
        *s = &s[c.len_utf8()..];
        Ok(c)
    })
    .as_parser_in_place()
}

fn match_char(
    find: char,
) -> impl for<'a> Parser<&'a str, Output = (), Error = FilterError<EmptyInput>> {
    any_char().filter(move |&c: &char| c == find).map(drop)
}

fn eat_white_space() -> impl for<'a> Parser<&'a str, Output = (), Error = util::Infallible> {
    any_char()
        .filter(|x: &char| x.is_whitespace())
        .zero_or_more(util::ignore)
        .map(drop)
}

use std::num::ParseFloatError;

#[derive(Debug)]
enum NumberError {
    ParseError(ParseFloatError),
    MalformedFloat,
}

impl From<ParseFloatError> for NumberError {
    fn from(p: ParseFloatError) -> Self {
        NumberError::ParseError(p)
    }
}

impl From<FoundZero> for NumberError {
    fn from(_: FoundZero) -> Self {
        NumberError::MalformedFloat
    }
}

fn number() -> impl for<'a> Parser<&'a str, Output = f64, Error = NumberError> {
    any_char()
        .filter(|x: &char| x.is_numeric())
        .one_or_more(String::new)
        .then(
            match_char('.')
                .then(
                    any_char()
                        .filter(|x: &char| x.is_numeric())
                        .one_or_more(String::new),
                )
                .map(util::snd)
                .optional(),
        )
        .map_err(util::unwrap_left)
        .map_err(Into::into)
        .flat_map(|(mut l, r): (String, Result<String, _>)| {
            if let Ok(r) = r {
                let r: String = r;
                l.push('.');
                l += &r;
            }
            Ok(l.parse()?)
        })
}

#[derive(Debug)]
enum StringError {
    NoStart,
    NoEnd,
}

impl From<Either<FilterError<EmptyInput>, FilterError<EmptyInput>>> for StringError {
    fn from(e: Either<FilterError<EmptyInput>, FilterError<EmptyInput>>) -> Self {
        match e {
            Either::Left(_) => StringError::NoStart,
            Either::Right(_) => StringError::NoEnd,
        }
    }
}

fn string() -> impl for<'a> Parser<&'a str, Output = String, Error = StringError> {
    match_char('"')
        .then(any_char().filter(|&x: &char| x != '"').zero_or_more(String::new))
        .map_both(util::snd, util::unwrap_left)
        .then(match_char('"'))
        .map(util::fst)
        .map_err(StringError::from)
}

#[derive(Debug)]
enum ItemError {
    Key(StringError),
    Colon,
    Value(Box<ValueError>),
}

impl From<Either<Either<StringError, FilterError<EmptyInput>>, ValueError>> for ItemError {
    fn from(e: Either<Either<StringError, FilterError<EmptyInput>>, ValueError>) -> Self {
        match e {
            Either::Left(Either::Left(x)) => ItemError::Key(x),
            Either::Left(Either::Right(_)) => ItemError::Colon,
            Either::Right(x) => ItemError::Value(Box::new(x)),
        }
    }
}

fn item() -> impl for<'a> Parser<&'a str, Output = (String, JsonValue), Error = ItemError> {
    string()
        .then(eat_white_space())
        .map_both(util::fst, util::unwrap_left)
        .then(match_char(':'))
        .map(util::fst)
        .then(eat_white_space())
        .map_both(util::fst, util::unwrap_left)
        .then(value())
        .then(eat_white_space())
        .map_both(util::fst, util::unwrap_left)
        .map_err(ItemError::from)
}

#[derive(Debug)]
enum ListError {
    NoStart,
    NoEnd,
}

impl From<Either<FilterError<EmptyInput>, FilterError<EmptyInput>>> for ListError {
    fn from(e: Either<FilterError<EmptyInput>, FilterError<EmptyInput>>) -> Self {
        match e {
            Either::Left(_) => ListError::NoStart,
            Either::Right(_) => ListError::NoEnd,
        }
    }
}

use parser_combinators::repeat::collections::Collection;

fn generalized_list<
    Output,
    Error,
    P: for<'a> Parser<&'a str, Output = Output, Error = Error>,
    Item,
    F,
    C,
>(
    start: char,
    end: char,
    sep: char,
    item: Item,
    f: F,
) -> impl for<'a> Parser<&'a str, Output = C, Error = ListError>
where
    C: Default + Collection<Output> + 'static,
    Item: Fn() -> P,
    F: Fn(Output) -> C + Copy,
{
    match_char(start)
        .then(eat_white_space())
        .map_both(util::fst, util::unwrap_left)
        .then(
            item()
                .and_then(move |x| {
                    match_char(sep)
                        .then(eat_white_space())
                        .map_both(util::fst, util::unwrap_left)
                        .then(item())
                        .map(util::snd)
                        .then(eat_white_space())
                        .map_both(util::fst, util::unwrap_left)
                        .zero_or_more(move || f(x))
                })
                .map_err(util::unwrap_left)
                .optional(),
        )
        .map_both(util::snd, util::unwrap_left)
        .then(match_char(end))
        .map(util::fst)
        .map_both(Result::unwrap_or_default, ListError::from)
}

fn object() -> impl for<'a> Parser<&'a str, Output = HashMap<String, JsonValue>, Error = ListError>
{
    generalized_list('{', '}', ',', item, |(id, value)| {
        let mut hm = HashMap::new();
        hm.insert(id, value);
        hm
    })
}

fn list() -> impl for<'a> Parser<&'a str, Output = Vec<JsonValue>, Error = ListError> {
    generalized_list('[', ']', ',', value, |x| vec![x])
}

#[derive(Debug)]
struct ValueError {
    number_error: NumberError,
    string_error: StringError,
    object_error: ListError,
    list_error: ListError,
}

impl From<(((NumberError, StringError), ListError), ListError)> for ValueError {
    fn from(
        (((number_error, string_error), object_error), list_error): (
            ((NumberError, StringError), ListError),
            ListError,
        ),
    ) -> Self {
        Self {
            number_error,
            string_error,
            object_error,
            list_error,
        }
    }
}

fn value() -> Box<dyn for<'a> Parser<&'a str, Output = JsonValue, Error = ValueError> + Send + Sync>
{
    // This box doesn't allocate, because the insides are zero-sized
    Box::new(
        defer(|| {
            number().map(JsonValue::from)
                .or(string().map(JsonValue::from))
                .map(Either::into_inner)
                .or(list().map(JsonValue::from))
                .map(Either::into_inner)
                .or(object().map(JsonValue::from))
                .map(Either::into_inner)
        })
        .map_err(ValueError::from),
    ) as Box<dyn for<'a> Parser<&'a str, Output = _, Error = _> + Send + Sync>
}

#[derive(Debug)]
enum JsonValue {
    Number(f64),
    String(String),
    Object(HashMap<String, JsonValue>),
    List(Vec<JsonValue>),
}

impl From<f64> for JsonValue {
    fn from(s: f64) -> Self {
        JsonValue::Number(s)
    }
}

impl From<String> for JsonValue {
    fn from(s: String) -> Self {
        JsonValue::String(s)
    }
}

impl From<HashMap<String, JsonValue>> for JsonValue {
    fn from(s: HashMap<String, JsonValue>) -> Self {
        JsonValue::Object(s)
    }
}

impl From<Vec<JsonValue>> for JsonValue {
    fn from(s: Vec<JsonValue>) -> Self {
        JsonValue::List(s)
    }
}

fn main() -> std::io::Result<()> {
    use std::fs::File;
    use std::io::Read;

    let mut file = File::open("E:/Programming/Rust/parser-combin/text.json")?;

    let mut doc = String::new();
    file.read_to_string(&mut doc)?;
    let doc: &str = &doc;

    let mut parser = value();

    let (doc, value) = parser.parse_mut(doc);
    println!("{:#?}", value);
    // let (doc, value) = parser.parse(doc);
    // println!("{:#?}", value);
    println!("{}", doc);

    Ok(())
}
