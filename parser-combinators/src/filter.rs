use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum FilterError<E> {
    ParseError(E),
    FilterError,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Filter<P, F>(pub(crate) P, pub(crate) F);

impl<Input: Clone, P: ParserOnce<Input>, F> ParserOnce<Input> for Filter<P, F>
where
    F: FnOnce(&P::Output) -> bool,
{
    type Output = P::Output;
    type Error = FilterError<P::Error>;

    #[inline]
    fn parse_once(self, input: Input) -> ParseResult<Input, Self> {
        let old_input = input.clone();
        let (input, out) = self.0.parse_once(input);
        match out {
            Ok(x) => {
                if (self.1)(&x) {
                    (input, Ok(x))
                } else {
                    (old_input, Err(FilterError::FilterError))
                }
            }
            Err(x) => (old_input, Err(FilterError::ParseError(x))),
        }
    }

    impl_parse_box! { Input }
}

impl<Input: Clone, P: ParserMut<Input>, F> ParserMut<Input> for Filter<P, F>
where
    F: FnMut(&P::Output) -> bool,
{
    #[inline]
    fn parse_mut(&mut self, input: Input) -> ParseResult<Input, Self> {
        let old_input = input.clone();
        let (input, out) = self.0.parse_mut(input);
        match out {
            Ok(x) => {
                if (self.1)(&x) {
                    (input, Ok(x))
                } else {
                    (old_input, Err(FilterError::FilterError))
                }
            }
            Err(x) => (old_input, Err(FilterError::ParseError(x))),
        }
    }
}

impl<Input: Clone, P: Parser<Input>, F> Parser<Input> for Filter<P, F>
where
    F: Fn(&P::Output) -> bool,
{
    #[inline]
    fn parse(&self, input: Input) -> ParseResult<Input, Self> {
        let old_input = input.clone();
        let (input, out) = self.0.parse(input);
        match out {
            Ok(x) => {
                if (self.1)(&x) {
                    (input, Ok(x))
                } else {
                    (old_input, Err(FilterError::FilterError))
                }
            }
            Err(x) => (old_input, Err(FilterError::ParseError(x))),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FilterInput<P, F>(pub(crate) P, pub(crate) F);

impl<Input, P: ParserOnce<Input>, F> ParserOnce<Input> for FilterInput<P, F>
where
    F: FnOnce(&Input) -> bool,
{
    type Output = P::Output;
    type Error = FilterError<P::Error>;

    #[inline]
    fn parse_once(self, input: Input) -> ParseResult<Input, Self> {
        if (self.1)(&input) {
            let (input, out) = self.0.parse_once(input);
            (input, out.map_err(FilterError::ParseError))
        } else {
            (input, Err(FilterError::FilterError))
        }
    }

    impl_parse_box! { Input }
}

impl<Input, P: ParserMut<Input>, F> ParserMut<Input> for FilterInput<P, F>
where
    F: FnMut(&Input) -> bool,
{
    #[inline]
    fn parse_mut(&mut self, input: Input) -> ParseResult<Input, Self> {
        if (self.1)(&input) {
            let (input, out) = self.0.parse_mut(input);
            (input, out.map_err(FilterError::ParseError))
        } else {
            (input, Err(FilterError::FilterError))
        }
    }
}

impl<Input, P: Parser<Input>, F> Parser<Input> for FilterInput<P, F>
where
    F: Fn(&Input) -> bool,
{
    #[inline]
    fn parse(&self, input: Input) -> ParseResult<Input, Self> {
        if (self.1)(&input) {
            let (input, out) = self.0.parse(input);
            (input, out.map_err(FilterError::ParseError))
        } else {
            (input, Err(FilterError::FilterError))
        }
    }
}
