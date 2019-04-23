use super::*;

use std::ops::RangeBounds;

pub mod collections;

use collections::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Optional<P>(pub(crate) P);

impl <Input, P> Parser<Input> for Optional<P>
where P: Parser<Input> {
    type Output = Result<P::Output, P::Error>;
    type Error = Infallible;

    fn parse(&mut self, input: Input) -> ParseResult<Input, Self> {
        let (input, out) = self.0.parse(input);
        (input, Ok(out))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ZeroOrMore<P, F>(pub(crate) P, pub(crate) F);

impl <Input, P, F, C> Parser<Input> for ZeroOrMore<P, F>
where P: Parser<Input>,
      F: FnMut() -> C,
      C: Collection<P::Output> {
    type Output = C;
    type Error = Infallible;

    fn parse(&mut self, mut input: Input) -> ParseResult<Input, Self> {
        let mut c = (self.1)();

        loop {
            let (next, out) = self.0.parse(input);
            input = next;

            match out {
                Ok(x) => c.put(x),
                Err(_) => break
            }
        }

        (input, Ok(c))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct OneOrMore<P, F>(pub(crate) ZeroOrMore<P, F>);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FoundZero;

impl<Input: Clone, P, F, C> Parser<Input> for OneOrMore<P, F>
where P: Parser<Input>,
      F: FnMut() -> C,
      C: Collection<P::Output> {
    type Output = C;
    type Error = FoundZero;

    fn parse(&mut self, input: Input) -> ParseResult<Input, Self> {
        let old_input = input.clone();
        let (input, out) = self.0.parse(input);

        let out = out.unwrap();

        if out.is_empty() {
            (old_input, Err(FoundZero))
        } else {
            (input, Ok(out))
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Repeat<P, F, R>(pub(crate) P, pub(crate) F, pub(crate) R);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct RangeError;

impl<Input: Clone, P, F, R, C> Parser<Input> for Repeat<P, F, R>
where P: Parser<Input>,
      R: RangeBounds<usize>,
      F: FnMut() -> C,
      C: Collection<P::Output> {
    type Output = C;
    type Error = RangeError;

    fn parse(&mut self, mut input: Input) -> ParseResult<Input, Self> {
        let mut c = (self.1)();

        use std::ops::Bound;

        let min = match self.2.start_bound() {
            Bound::Unbounded => 0,
            Bound::Included(&x) => x + 1,
            Bound::Excluded(&x) => x,
        };

        let max = match self.2.end_bound() {
            Bound::Unbounded => usize::max_value(),
            Bound::Included(&x) => x + 1,
            Bound::Excluded(&x) => x,
        };

        let old_input = input.clone();

        for _ in 1..min {
            let (next, out) = self.0.parse(input);
            input = next;

            match out {
                Ok(x) => c.put(x),
                Err(_) => return (old_input, Err(RangeError))
            }
        }

        for _ in min..=max {
            let (next, out) = self.0.parse(input);
            input = next;

            match out {
                Ok(x) => c.put(x),
                Err(_) => break
            }
        }

        (input, Ok(c))
    }
}