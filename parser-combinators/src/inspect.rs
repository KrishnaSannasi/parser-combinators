use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Inspect<P, F>(pub(crate) P, pub(crate) F);

impl<Input, P: ParserOnce<Input>, F> ParserOnce<Input> for Inspect<P, F>
where
    F: FnOnce(&Result<P::Output, P::Error>),
{
    type Output = P::Output;
    type Error = P::Error;

    #[inline]
    fn parse_once(self, input: Input) -> ParseResult<Input, Self> {
        let (input, out) = self.0.parse_once(input);
        (self.1)(&out);
        (input, out)
    }

    impl_parse_box! { Input }
}

impl<Input, P: ParserMut<Input>, F> ParserMut<Input> for Inspect<P, F>
where
    F: FnMut(&Result<P::Output, P::Error>),
{
    #[inline]
    fn parse_mut(&mut self, input: Input) -> ParseResult<Input, Self> {
        let (input, out) = self.0.parse_mut(input);
        (self.1)(&out);
        (input, out)
    }
}

impl<Input, P: Parser<Input>, F> Parser<Input> for Inspect<P, F>
where
    F: Fn(&Result<P::Output, P::Error>),
{
    #[inline]
    fn parse(&self, input: Input) -> ParseResult<Input, Self> {
        let (input, out) = self.0.parse(input);
        (self.1)(&out);
        (input, out)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct InspectInput<P, F>(pub(crate) P, pub(crate) F);

impl<Input, P: ParserOnce<Input>, F> ParserOnce<Input> for InspectInput<P, F>
where
    F: FnOnce(&Input),
{
    type Output = P::Output;
    type Error = P::Error;

    #[inline]
    fn parse_once(self, input: Input) -> ParseResult<Input, Self> {
        let (input, out) = self.0.parse_once(input);
        (self.1)(&input);
        (input, out)
    }

    impl_parse_box! { Input }
}

impl<Input, P: ParserMut<Input>, F> ParserMut<Input> for InspectInput<P, F>
where
    F: FnMut(&Input),
{
    #[inline]
    fn parse_mut(&mut self, input: Input) -> ParseResult<Input, Self> {
        let (input, out) = self.0.parse_mut(input);
        (self.1)(&input);
        (input, out)
    }
}

impl<Input, P: Parser<Input>, F> Parser<Input> for InspectInput<P, F>
where
    F: Fn(&Input),
{
    #[inline]
    fn parse(&self, input: Input) -> ParseResult<Input, Self> {
        let (input, out) = self.0.parse(input);
        (self.1)(&input);
        (input, out)
    }
}
