#![deny(clippy::pedantic)]
#![deny(clippy::all)]
#![allow(non_camel_case_types)]

use parser_combinators::prelude::*; 
use parser_combinators::parse::*;

use parser_combinators::filter::FilterError;
use parser_combinators::repeat::FoundZero;

use either::Either;

use std::convert::Infallible;

#[derive(Debug)]
struct LiteralError(&'static str);
fn match_literal(expected: &'static str) -> impl for<'r> Parser<&'r str, Output = (), Error = LiteralError> + Copy {
    (move |input: &mut &str| {
        if input.starts_with(expected) {
            *input = &input[expected.len()..];
            Ok(())
        } else {
            Err(LiteralError(expected))
        }
    }).as_parser_in_place()
}

#[derive(Debug)]
pub struct EmptyInput;
fn any_char() -> impl for<'a> Parser<&'a str, Output = char, Error = EmptyInput> + Copy {
    (|input: &mut &str| {
        let c = input.chars().next().ok_or(EmptyInput)?;
        *input = &input[c.len_utf8()..];
        Ok(c)
    }).as_parser_in_place()
}

#[derive(Debug)]
pub struct InvalidIdent;
fn identifier() -> impl for<'a> Parser<&'a str, Output = String, Error = InvalidIdent> + Copy {
    (|input: &mut &str| {
        let mut matched = String::new();
        let mut chars = input.chars();

        match chars.next() {
            Some(next) if next.is_alphabetic() => matched.push(next),
            _ => return Err(InvalidIdent),
        }

        for next in chars {
            if next.is_alphanumeric() || next == '-' {
                matched.push(next);
            } else {
                break;
            }
        }

        let next_index = matched.len();
        *input = &input[next_index..];
        Ok(matched)
    }).as_parser_in_place()
}

#[derive(Debug)]
pub enum MissingQuote {
    First,
    Second
}

impl From<Either<LiteralError, LiteralError>> for MissingQuote {
    fn from(e: Either<LiteralError, LiteralError>) -> Self {
        match e {
            Either::Left(LiteralError(_)) => MissingQuote::First,
            Either::Right(LiteralError(_)) => MissingQuote::Second,
        }
    }
}

fn quoted_string() -> impl for<'a> Parser<&'a str, Output = String, Error = MissingQuote> + Copy {
    match_literal("\"").then(
        any_char()
            .filter(|&c: &char| c != '"')
            .zero_or_more(String::new)
    ).map_both(util::snd, util::unwrap_left)
        .then(match_literal("\"")).map(util::fst)
        .map_err(MissingQuote::from)
}

fn whitespace_char() -> impl for<'a> Parser<&'a str, Output = char, Error = FilterError<EmptyInput>> + Copy {
    #[allow(clippy::trivially_copy_pass_by_ref)]
    any_char()
        .filter(|x| x.is_whitespace())
}

fn eat_white_space() -> impl for<'a> Parser<&'a str, Output = (), Error = Infallible> + Copy {
    whitespace_char()
        .zero_or_more(util::ignore)
}

#[derive(Debug)]
enum AttributeError {
    MissingWhitespace,
    MissingIdent,
    MissingEqual,
    MissingValue
}

impl From<Either<Either<Either<FoundZero, InvalidIdent>, LiteralError>, MissingQuote>> for AttributeError {
    fn from(e: Either<Either<Either<FoundZero, InvalidIdent>, LiteralError>, MissingQuote>) -> Self {
        match e {
            Either::Left(Either::Left(Either::Left(_))) => AttributeError::MissingWhitespace,
            Either::Left(Either::Left(Either::Right(_))) => AttributeError::MissingIdent,
            Either::Left(Either::Right(_)) => AttributeError::MissingEqual,
            Either::Right(_) => AttributeError::MissingValue
        }
    }
}

fn attribute() -> impl for<'a> Parser<&'a str, Output = (String, String), Error = AttributeError> {
    whitespace_char()
        .one_or_more(util::count)
        .then(identifier()).map(util::snd)
        .then(eat_white_space()).map_both(util::fst, util::unwrap_left)
        .then(match_literal("=")).map(util::fst)
        .then(eat_white_space()).map_both(util::fst, util::unwrap_left)
        .then(quoted_string())
        .map_err(AttributeError::from)
}

#[derive(Debug)]
enum SingleElementError {
    MissingStart,
    InvalidIdent,
    MissingEnd
}

impl From<Either<Either<LiteralError, InvalidIdent>, LiteralError>> for SingleElementError {
    fn from(e: Either<Either<LiteralError, InvalidIdent>, LiteralError>) -> Self {
        match e {
            Either::Left(Either::Left(_)) => SingleElementError::MissingStart,
            Either::Left(Either::Right(_)) => SingleElementError::InvalidIdent,
            Either::Right(_) => SingleElementError::MissingEnd,
        }
    }
}

fn single_element() -> impl for<'a> Parser<&'a str, Output = Element, Error = SingleElementError> {
    match_literal("<")
        .then(identifier()).map(util::snd)
        .then(attribute().zero_or_more(Vec::new)).map_err(util::unwrap_left)
        .then(eat_white_space()).map_both(util::fst, util::unwrap_left)
        .then(match_literal("/>"))
        .map_both(
            |((name, attributes), _)| Element::Node { name, attributes, children: Vec::new() },
            SingleElementError::from
        )
}

#[derive(Debug)]
enum ParentElementError {
    Open(SingleElementError),
    Close(SingleElementError),
    WrongCloseTag
}

// Either<
// Either<LiteralError, InvalidIdent>, Either<Either<Either<LiteralError, LiteralError>, FilterError<InvalidIdent>>, LiteralError>>

impl From<Either<Either<Either<LiteralError, InvalidIdent>, LiteralError>, Either<Either<LiteralError, FilterError<InvalidIdent>>, LiteralError>>> for ParentElementError {
    fn from(e: Either<Either<Either<LiteralError, InvalidIdent>, LiteralError>,
               Either<Either<LiteralError, FilterError<InvalidIdent>>, LiteralError>>) -> Self {
        match e {
            Either::Left(e) => ParentElementError::Open(e.into()),
            Either::Right(Either::Left(Either::Left(_))) => ParentElementError::Close(SingleElementError::MissingStart),
            Either::Right(Either::Left(Either::Right(FilterError::ParseError(_)))) => ParentElementError::Close(SingleElementError::InvalidIdent),
            Either::Right(Either::Left(Either::Right(FilterError::FilterError))) => ParentElementError::WrongCloseTag,
            Either::Right(Either::Right(_)) => ParentElementError::Close(SingleElementError::MissingEnd),
        }
    }
}

fn parent_element() -> impl for<'a> Parser<&'a str, Output = Element, Error = ParentElementError> {
    match_literal("<")
        .then(identifier()).map(util::snd)
        .then(attribute().zero_or_more(Vec::new)).map_err(util::unwrap_left)
        .then(match_literal(">")).map(util::fst)
        .then(eat_white_space()).map_both(util::fst, util::unwrap_left)

        .and_then(|(ident, attributes)| {
            let name = ident.clone();
            parser_combinators::parse_once::ParserCombinators::map(
                
                element()
                    .then(eat_white_space()).map_both(util::fst, util::unwrap_left)
                    .zero_or_more(Vec::new)

                    .then(match_literal("</")).map_both(util::fst, util::unwrap_right)
                    .then(identifier().filter(move |i| i == &ident)).map(util::fst)
                    .then(match_literal(">")).map(util::fst),
                move |children| {
                    Element::Node {
                        name, attributes, children
                    }
                }
            )
        })
        .map_err(ParentElementError::from)

}

#[derive(Debug)]
struct ElementError((SingleElementError, ParentElementError));

fn element() -> impl for<'a> Parser<&'a str, Output = Element, Error = ElementError> {
    Box::new(defer(|| {
        single_element()
            .or(parent_element()).map(Either::into_inner)
            .map_err(ElementError)
    })) as Box<dyn for<'a> Parser<&'a str, Output = _, Error = _>>
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Element {
    Comment(String),
    Node {
        name: String,
        attributes: Vec<(String, String)>,
        children: Vec<Element>,
    },
}

fn main() -> std::io::Result<()> {
    use std::fs::File;
    use std::io::Read;

    let mut file = File::open("../text.xml")?;

    let mut doc = String::new();
    file.read_to_string(&mut doc)?;

    let parser = element();
    let (doc, parsed_doc) = parser.parse(&doc);
    
    println!("{:#?}", parsed_doc);

    println!("{}", doc);
    
    // assert_eq!(Ok(("", parsed_doc)), element().parse(doc));

    Ok(())
}
