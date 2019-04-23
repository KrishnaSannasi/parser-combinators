use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum FilterError<E> {
    ParseError(E),
    FilterError
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Filter<P, F>(pub(crate) P, pub(crate) F);

impl<Input: Clone, P: Parser<Input>, F> Parser<Input> for Filter<P, F>
where F: FnMut(&P::Output) -> bool {
    type Output = P::Output;
    type Error = FilterError<P::Error>;

    fn parse(&mut self, input: Input) -> ParseResult<Input, Self> {
        let old_input = input.clone();
        let (input, out) = self.0.parse(input);
        match out {
            Ok(x) => if (self.1)(&x) {
                (input, Ok(x))
            } else {
                (old_input, Err(FilterError::FilterError))
            },
            Err(x) => (old_input, Err(FilterError::ParseError(x)))
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FilterInput<P, F>(pub(crate) P, pub(crate) F);

impl<Input, P: Parser<Input>, F> Parser<Input> for FilterInput<P, F>
where F: FnMut(&Input) -> bool {
    type Output = P::Output;
    type Error = FilterError<P::Error>;

    fn parse(&mut self, input: Input) -> ParseResult<Input, Self> {
        if (self.1)(&input) {
            let (input, out) = self.0.parse(input);
            (input, out.map_err(FilterError::ParseError))
        } else {
            (input, Err(FilterError::FilterError))
        }
    }
}
