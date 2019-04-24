use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Map<P, F>(pub(crate) P, pub(crate) F);

impl<Input, P: ParserOnce<Input>, F, Output> ParserOnce<Input> for Map<P, F>
where F: FnOnce(P::Output) -> Output {
    type Output = Output;
    type Error = P::Error;

    fn parse_once(self, input: Input) -> ParseResult<Input, Self> {
        let (input, out) = self.0.parse_once(input);
        (input, out.map(self.1))
    }
}

impl<Input, P: ParserMut<Input>, F, Output> ParserMut<Input> for Map<P, F>
where F: FnMut(P::Output) -> Output {
    fn parse_mut(&mut self, input: Input) -> ParseResult<Input, Self> {
        let (input, out) = self.0.parse_mut(input);
        (input, out.map(&mut self.1))
    }
}

impl<Input, P: Parser<Input>, F, Output> Parser<Input> for Map<P, F>
where F: Fn(P::Output) -> Output {
    fn parse(&self, input: Input) -> ParseResult<Input, Self> {
        let (input, out) = self.0.parse(input);
        (input, out.map(&self.1))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct MapErr<P, F>(pub(crate) P, pub(crate) F);

impl<Input, P: ParserOnce<Input>, F, Error> ParserOnce<Input> for MapErr<P, F>
where F: FnOnce(P::Error) -> Error {
    type Output = P::Output;
    type Error = Error;

    fn parse_once(self, input: Input) -> ParseResult<Input, Self> {
        let (input, out) = self.0.parse_once(input);
        (input, out.map_err(self.1))
    }
}

impl<Input, P: ParserMut<Input>, F, Error> ParserMut<Input> for MapErr<P, F>
where F: FnMut(P::Error) -> Error {
    fn parse_mut(&mut self, input: Input) -> ParseResult<Input, Self> {
        let (input, out) = self.0.parse_mut(input);
        (input, out.map_err(&mut self.1))
    }
}

impl<Input, P: Parser<Input>, F, Error> Parser<Input> for MapErr<P, F>
where F: Fn(P::Error) -> Error {
    fn parse(&self, input: Input) -> ParseResult<Input, Self> {
        let (input, out) = self.0.parse(input);
        (input, out.map_err(&self.1))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct MapBoth<P, F, G>(pub(crate) P, pub(crate) F, pub(crate) G);

impl<Input, P: ParserOnce<Input>, F, G, Output, Error> ParserOnce<Input> for MapBoth<P, F, G>
where F: FnOnce(P::Output) -> Output,
      G: FnOnce(P::Error) -> Error {
    type Output = Output;
    type Error = Error;
    
    fn parse_once(self, input: Input) -> ParseResult<Input, Self> {
        let (input, out) = self.0.parse_once(input);
        (
            input,
            out.map(self.1).map_err(self.2)
        )
    }
}

impl<Input, P: ParserMut<Input>, F, G, Output, Error> ParserMut<Input> for MapBoth<P, F, G>
where F: FnMut(P::Output) -> Output,
      G: FnMut(P::Error) -> Error {
    fn parse_mut(&mut self, input: Input) -> ParseResult<Input, Self> {
        let (input, out) = self.0.parse_mut(input);
        (
            input,
            out.map(&mut self.1).map_err(&mut self.2)
        )
    }
}

impl<Input, P: Parser<Input>, F, G, Output, Error> Parser<Input> for MapBoth<P, F, G>
where F: Fn(P::Output) -> Output,
      G: Fn(P::Error) -> Error {
    fn parse(&self, input: Input) -> ParseResult<Input, Self> {
        let (input, out) = self.0.parse(input);
        (
            input,
            out.map(&self.1).map_err(&self.2)
        )
    }
}
