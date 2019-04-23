use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Inspect<P, F>(pub(crate) P, pub(crate) F);

impl<Input, P: Parser<Input>, F> Parser<Input> for Inspect<P, F>
where F: FnMut(&Result<P::Output, P::Error>) {
    type Output = P::Output;
    type Error = P::Error;

    fn parse(&mut self, input: Input) -> ParseResult<Input, Self> {
        let (input, out) = self.0.parse(input);
        (self.1)(&out);
        (input, out)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct InspectInput<P, F>(pub(crate) P, pub(crate) F);

impl<Input, P: Parser<Input>, F> Parser<Input> for InspectInput<P, F>
where F: FnMut(&Input) {
    type Output = P::Output;
    type Error = P::Error;

    fn parse(&mut self, input: Input) -> ParseResult<Input, Self> {
        let (input, out) = self.0.parse(input);
        (self.1)(&input);
        (input, out)
    }
}
