use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Map<P, F>(pub(crate) P, pub(crate) F);

impl<Input, P: Parser<Input>, F, Output> Parser<Input> for Map<P, F>
where F: FnMut(P::Output) -> Output {
    type Output = Output;
    type Error = P::Error;

    fn parse(&mut self, input: Input) -> ParseResult<Input, Self> {
        let (input, out) = self.0.parse(input);
        (input, out.map(&mut self.1))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct MapErr<P, F>(pub(crate) P, pub(crate) F);

impl<Input, P: Parser<Input>, F, Error> Parser<Input> for MapErr<P, F>
where F: FnMut(P::Error) -> Error {
    type Output = P::Output;
    type Error = Error;

    fn parse(&mut self, input: Input) -> ParseResult<Input, Self> {
        let (input, out) = self.0.parse(input);
        (input, out.map_err(&mut self.1))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct MapBoth<P, F, G>(pub(crate) P, pub(crate) F, pub(crate) G);

impl<Input, P: Parser<Input>, F, G, Output, Error> Parser<Input> for MapBoth<P, F, G>
where F: FnMut(P::Output) -> Output,
      G: FnMut(P::Error) -> Error {
    type Output = Output;
    type Error = Error;

    fn parse(&mut self, input: Input) -> ParseResult<Input, Self> {
        let (input, out) = self.0.parse(input);
        (
            input,
            out.map(&mut self.1).map_err(&mut self.2)
        )
    }
}
