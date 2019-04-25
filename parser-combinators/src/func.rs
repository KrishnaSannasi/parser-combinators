use super::*;

pub trait AsParser: Sized {
    #[inline]
    fn defer_once<Input>(self) -> DeferOnce<Self>
    where
        DeferOnce<Self>: ParserOnce<Input>,
    {
        DeferOnce(self)
    }
    #[inline]
    fn as_parser_once<Input>(self) -> FuncOnce<Self>
    where
        FuncOnce<Self>: ParserOnce<Input>,
    {
        FuncOnce(self)
    }
    #[inline]
    fn as_parser_once_in_place<Input>(self) -> FuncOnceInPlace<Self>
    where
        FuncOnceInPlace<Self>: ParserOnce<Input>,
    {
        FuncOnceInPlace(self)
    }

    #[inline]
    fn defer_mut<Input>(self) -> DeferMut<Self>
    where
        DeferMut<Self>: ParserMut<Input>,
    {
        DeferMut(self)
    }
    #[inline]
    fn as_parser_mut<Input>(self) -> FuncMut<Self>
    where
        FuncMut<Self>: ParserMut<Input>,
    {
        FuncMut(self)
    }
    #[inline]
    fn as_parser_mut_in_place<Input>(self) -> FuncMutInPlace<Self>
    where
        FuncMutInPlace<Self>: ParserMut<Input>,
    {
        FuncMutInPlace(self)
    }

    #[inline]
    fn defer<Input>(self) -> Defer<Self>
    where
        Defer<Self>: Parser<Input>,
    {
        Defer(self)
    }
    #[inline]
    fn as_parser<Input>(self) -> Func<Self>
    where
        Func<Self>: Parser<Input>,
    {
        Func(self)
    }
    #[inline]
    fn as_parser_in_place<Input>(self) -> FuncInPlace<Self>
    where
        FuncInPlace<Self>: Parser<Input>,
    {
        FuncInPlace(self)
    }
}

impl<T> AsParser for T {}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FuncOnce<F: ?Sized>(F);

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FuncMut<F: ?Sized>(F);

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Func<F: ?Sized>(F);

impl<Input, F, Output, Error> ParserOnce<Input> for FuncOnce<F>
where
    F: FnOnce(Input) -> (Input, Result<Output, Error>),
{
    type Output = Output;
    type Error = Error;

    #[inline]
    fn parse_once(self, input: Input) -> ParseResult<Input, Self> {
        (self.0)(input)
    }

    #[inline]
    fn parse_box(self: Box<Self>, _input: Input) -> ParseResult<Input, Self> {
        unimplemented!(
            "`Box<dyn FnOnce(..) -> _>` cannot be called, so it cannot be used as a parser"
        )
    }
}

impl<Input, F, Output, Error> ParserOnce<Input> for FuncMut<F>
where
    F: ?Sized + FnMut(Input) -> (Input, Result<Output, Error>),
{
    type Output = Output;
    type Error = Error;

    #[inline]
    fn parse_once(mut self, input: Input) -> ParseResult<Input, Self>
    where
        Self: Sized,
    {
        self.parse_mut(input)
    }

    #[inline]
    fn parse_box(mut self: Box<Self>, input: Input) -> ParseResult<Input, Self> {
        self.parse_mut(input)
    }
}

impl<Input, F, Output, Error> ParserMut<Input> for FuncMut<F>
where
    F: ?Sized + FnMut(Input) -> (Input, Result<Output, Error>),
{
    #[inline]
    fn parse_mut(&mut self, input: Input) -> ParseResult<Input, Self> {
        (self.0)(input)
    }
}

impl<Input, F, Output, Error> ParserOnce<Input> for Func<F>
where
    F: ?Sized + Fn(Input) -> (Input, Result<Output, Error>),
{
    type Output = Output;
    type Error = Error;

    #[inline]
    fn parse_once(self, input: Input) -> ParseResult<Input, Self>
    where
        Self: Sized,
    {
        self.parse(input)
    }

    #[inline]
    fn parse_box(self: Box<Self>, input: Input) -> ParseResult<Input, Self> {
        self.parse(input)
    }
}

impl<Input, F, Output, Error> ParserMut<Input> for Func<F>
where
    F: ?Sized + Fn(Input) -> (Input, Result<Output, Error>),
{
    #[inline]
    fn parse_mut(&mut self, input: Input) -> ParseResult<Input, Self> {
        self.parse(input)
    }
}

impl<Input, F, Output, Error> Parser<Input> for Func<F>
where
    F: ?Sized + Fn(Input) -> (Input, Result<Output, Error>),
{
    #[inline]
    fn parse(&self, input: Input) -> ParseResult<Input, Self> {
        (self.0)(input)
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FuncOnceInPlace<F: ?Sized>(pub F);

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FuncMutInPlace<F: ?Sized>(pub F);

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FuncInPlace<F: ?Sized>(pub F);

impl<Input, F, Output, Error> ParserOnce<Input> for FuncOnceInPlace<F>
where
    F: FnOnce(&mut Input) -> Result<Output, Error>,
{
    type Output = Output;
    type Error = Error;

    #[inline]
    fn parse_once(self, mut input: Input) -> ParseResult<Input, Self>
    where
        Self: Sized,
    {
        let out = (self.0)(&mut input);
        (input, out)
    }

    impl_parse_box! { Input }
}

impl<Input, F, Output, Error> ParserOnce<Input> for FuncMutInPlace<F>
where
    F: ?Sized + FnMut(&mut Input) -> Result<Output, Error>,
{
    type Output = Output;
    type Error = Error;

    #[inline]
    fn parse_once(mut self, input: Input) -> ParseResult<Input, Self>
    where
        Self: Sized,
    {
        self.parse_mut(input)
    }

    fn parse_box(mut self: Box<Self>, input: Input) -> ParseResult<Input, Self> {
        self.parse_mut(input)
    }
}

impl<Input, F, Output, Error> ParserMut<Input> for FuncMutInPlace<F>
where
    F: ?Sized + FnMut(&mut Input) -> Result<Output, Error>,
{
    #[inline]
    fn parse_mut(&mut self, mut input: Input) -> ParseResult<Input, Self> {
        let out = (self.0)(&mut input);
        (input, out)
    }
}

impl<Input, F, Output, Error> ParserOnce<Input> for FuncInPlace<F>
where
    F: ?Sized + Fn(&mut Input) -> Result<Output, Error>,
{
    type Output = Output;
    type Error = Error;

    #[inline]
    fn parse_once(self, input: Input) -> ParseResult<Input, Self>
    where
        Self: Sized,
    {
        self.parse(input)
    }

    fn parse_box(self: Box<Self>, input: Input) -> ParseResult<Input, Self> {
        self.parse(input)
    }
}

impl<Input, F, Output, Error> ParserMut<Input> for FuncInPlace<F>
where
    F: ?Sized + Fn(&mut Input) -> Result<Output, Error>,
{
    #[inline]
    fn parse_mut(&mut self, input: Input) -> ParseResult<Input, Self> {
        self.parse(input)
    }
}

impl<Input, F, Output, Error> Parser<Input> for FuncInPlace<F>
where
    F: ?Sized + Fn(&mut Input) -> Result<Output, Error>,
{
    #[inline]
    fn parse(&self, mut input: Input) -> ParseResult<Input, Self> {
        let out = (self.0)(&mut input);
        (input, out)
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct DeferOnce<F: ?Sized>(pub F);

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct DeferMut<F: ?Sized>(pub F);

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Defer<F: ?Sized>(pub F);

impl<Input, F, P> ParserOnce<Input> for DeferOnce<F>
where
    F: FnOnce() -> P,
    P: ParserOnce<Input>,
{
    type Output = P::Output;
    type Error = P::Error;

    #[inline]
    fn parse_once(self, input: Input) -> ParseResult<Input, Self> {
        (self.0)().parse_once(input)
    }

    impl_parse_box! { Input }
}

impl<Input, F, P> ParserOnce<Input> for DeferMut<F>
where
    F: ?Sized + FnMut() -> P,
    P: ParserOnce<Input>,
{
    type Output = P::Output;
    type Error = P::Error;

    #[inline]
    fn parse_once(mut self, input: Input) -> ParseResult<Input, Self>
    where
        Self: Sized,
    {
        self.parse_mut(input)
    }

    fn parse_box(mut self: Box<Self>, input: Input) -> ParseResult<Input, Self> {
        self.parse_mut(input)
    }
}

impl<Input, F, P> ParserMut<Input> for DeferMut<F>
where
    F: ?Sized + FnMut() -> P,
    P: ParserOnce<Input>,
{
    #[inline]
    fn parse_mut(&mut self, input: Input) -> ParseResult<Input, Self> {
        (self.0)().parse_once(input)
    }
}

impl<Input, F, P> ParserOnce<Input> for Defer<F>
where
    F: ?Sized + Fn() -> P,
    P: ParserOnce<Input>,
{
    type Output = P::Output;
    type Error = P::Error;

    #[inline]
    fn parse_once(self, input: Input) -> ParseResult<Input, Self>
    where
        Self: Sized,
    {
        self.parse(input)
    }

    fn parse_box(self: Box<Self>, input: Input) -> ParseResult<Input, Self> {
        self.parse(input)
    }
}

impl<Input, F, P> ParserMut<Input> for Defer<F>
where
    F: ?Sized + Fn() -> P,
    P: ParserOnce<Input>,
{
    #[inline]
    fn parse_mut(&mut self, input: Input) -> ParseResult<Input, Self> {
        self.parse(input)
    }
}

impl<Input, F, P> Parser<Input> for Defer<F>
where
    F: ?Sized + Fn() -> P,
    P: ParserOnce<Input>,
{
    #[inline]
    fn parse(&self, input: Input) -> ParseResult<Input, Self> {
        (self.0)().parse_once(input)
    }
}
