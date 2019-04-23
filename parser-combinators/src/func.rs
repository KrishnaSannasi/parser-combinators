use super::*;

pub trait AsParser: Sized {
    fn as_parser<Input>(self) -> Func<Self> where Func<Self>: Parser<Input> { Func(self) }
    fn as_parser_in_place<Input>(self) -> FuncInPlace<Self> where FuncInPlace<Self>: Parser<Input> { FuncInPlace(self) }
    fn defer<Input>(self) -> Defer<Self> where Defer<Self>: Parser<Input> { Defer(self) }
}

impl<T> AsParser for T {}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Func<F: ?Sized>(pub F);

impl<Input, F, Output, Error> Parser<Input> for Func<F>
where F: ?Sized + FnMut(Input) -> (Input, Result<Output, Error>) {
    type Output = Output;
    type Error = Error;

    fn parse(&mut self, input: Input) -> ParseResult<Input, Self> {
        (self.0)(input)
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FuncInPlace<F: ?Sized>(pub F);

impl<Input, F, Output, Error> Parser<Input> for FuncInPlace<F>
where F: ?Sized + FnMut(&mut Input) -> Result<Output, Error> {
    type Output = Output;
    type Error = Error;

    fn parse(&mut self, mut input: Input) -> ParseResult<Input, Self> {
        let out = (self.0)(&mut input);
        (input, out)
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Defer<F: ?Sized>(pub F);

impl<Input, F, P> Parser<Input> for Defer<F>
where F: ?Sized + FnMut() -> P,
      P: Parser<Input> {
    type Output = P::Output;
    type Error = P::Error;

    fn parse(&mut self, input: Input) -> ParseResult<Input, Self> {
        (self.0)().parse(input)
    }
}