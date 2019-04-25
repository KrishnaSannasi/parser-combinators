use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FlatMap<P, F>(pub(crate) P, pub(crate) F);

impl<Input, P: ParserOnce<Input>, F, Output> ParserOnce<Input> for FlatMap<P, F>
where
    F: FnOnce(P::Output) -> Result<Output, P::Error>,
{
    type Output = Output;
    type Error = P::Error;

    #[inline]
    fn parse_once(self, input: Input) -> ParseResult<Input, Self> {
        let (input, out) = self.0.parse_once(input);
        (input, out.and_then(self.1))
    }

    impl_parse_box! { Input }
}

impl<Input, P: ParserMut<Input>, F, Output> ParserMut<Input> for FlatMap<P, F>
where
    F: FnMut(P::Output) -> Result<Output, P::Error>,
{
    #[inline]
    fn parse_mut(&mut self, input: Input) -> ParseResult<Input, Self> {
        let (input, out) = self.0.parse_mut(input);
        (input, out.and_then(&mut self.1))
    }
}

impl<Input, P: Parser<Input>, F, Output> Parser<Input> for FlatMap<P, F>
where
    F: Fn(P::Output) -> Result<Output, P::Error>,
{
    #[inline]
    fn parse(&self, input: Input) -> ParseResult<Input, Self> {
        let (input, out) = self.0.parse(input);
        (input, out.and_then(&self.1))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FlatMapErr<P, F>(pub(crate) P, pub(crate) F);

impl<Input, P: ParserOnce<Input>, F, Error> ParserOnce<Input> for FlatMapErr<P, F>
where
    F: FnOnce(P::Error) -> Result<P::Output, Error>,
{
    type Output = P::Output;
    type Error = Error;

    #[inline]
    fn parse_once(self, input: Input) -> ParseResult<Input, Self> {
        let (input, out) = self.0.parse_once(input);
        (input, out.or_else(self.1))
    }

    impl_parse_box! { Input }
}

impl<Input, P: ParserMut<Input>, F, Error> ParserMut<Input> for FlatMapErr<P, F>
where
    F: FnMut(P::Error) -> Result<P::Output, Error>,
{
    #[inline]
    fn parse_mut(&mut self, input: Input) -> ParseResult<Input, Self> {
        let (input, out) = self.0.parse_mut(input);
        (input, out.or_else(&mut self.1))
    }
}

impl<Input, P: Parser<Input>, F, Error> Parser<Input> for FlatMapErr<P, F>
where
    F: Fn(P::Error) -> Result<P::Output, Error>,
{
    #[inline]
    fn parse(&self, input: Input) -> ParseResult<Input, Self> {
        let (input, out) = self.0.parse(input);
        (input, out.or_else(&self.1))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FlatMapBoth<P, F, G>(pub(crate) P, pub(crate) F, pub(crate) G);

impl<Input, P: ParserOnce<Input>, F, G, Output, Error> ParserOnce<Input> for FlatMapBoth<P, F, G>
where
    F: FnOnce(P::Output) -> Result<Output, Error>,
    G: FnOnce(P::Error) -> Result<Output, Error>,
{
    type Output = Output;
    type Error = Error;

    #[inline]
    fn parse_once(self, input: Input) -> ParseResult<Input, Self> {
        let (input, out) = self.0.parse_once(input);

        (
            input,
            match out {
                Ok(x) => (self.1)(x),
                Err(x) => (self.2)(x),
            },
        )
    }

    impl_parse_box! { Input }
}

impl<Input, P: ParserMut<Input>, F, G, Output, Error> ParserMut<Input> for FlatMapBoth<P, F, G>
where
    F: FnMut(P::Output) -> Result<Output, Error>,
    G: FnMut(P::Error) -> Result<Output, Error>,
{
    #[inline]
    fn parse_mut(&mut self, input: Input) -> ParseResult<Input, Self> {
        let (input, out) = self.0.parse_mut(input);

        (
            input,
            match out {
                Ok(x) => (self.1)(x),
                Err(x) => (self.2)(x),
            },
        )
    }
}

impl<Input, P: Parser<Input>, F, G, Output, Error> Parser<Input> for FlatMapBoth<P, F, G>
where
    F: Fn(P::Output) -> Result<Output, Error>,
    G: Fn(P::Error) -> Result<Output, Error>,
{
    #[inline]
    fn parse(&self, input: Input) -> ParseResult<Input, Self> {
        let (input, out) = self.0.parse(input);

        (
            input,
            match out {
                Ok(x) => (self.1)(x),
                Err(x) => (self.2)(x),
            },
        )
    }
}
