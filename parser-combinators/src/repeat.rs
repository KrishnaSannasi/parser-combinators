use super::*;

use std::ops::RangeBounds;

pub mod collections;

use collections::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Optional<P>(pub(crate) P);

impl<Input, P> ParserOnce<Input> for Optional<P>
where
    P: ParserOnce<Input>,
{
    type Output = Result<P::Output, P::Error>;
    type Error = Infallible;

    #[inline]
    fn parse_once(self, input: Input) -> ParseResult<Input, Self> {
        let (input, out) = self.0.parse_once(input);
        (input, Ok(out))
    }

    impl_parse_box! { Input }
}

impl<Input, P> ParserMut<Input> for Optional<P>
where
    P: ParserMut<Input>,
{
    #[inline]
    fn parse_mut(&mut self, input: Input) -> ParseResult<Input, Self> {
        let (input, out) = self.0.parse_mut(input);
        (input, Ok(out))
    }
}

impl<Input, P> Parser<Input> for Optional<P>
where
    P: Parser<Input>,
{
    #[inline]
    fn parse(&self, input: Input) -> ParseResult<Input, Self> {
        let (input, out) = self.0.parse(input);
        (input, Ok(out))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ZeroOrMore<P, F>(pub(crate) P, pub(crate) F);

impl<Input, P, F, C> ParserOnce<Input> for ZeroOrMore<P, F>
where
    P: ParserMut<Input>,
    F: FnOnce() -> C,
    C: Collection<P::Output>,
{
    type Output = C;
    type Error = Infallible;

    #[inline]
    fn parse_once(mut self, mut input: Input) -> ParseResult<Input, Self> {
        let mut c = (self.1)();

        loop {
            let (next, out) = self.0.parse_mut(input);
            input = next;

            match out {
                Ok(x) => c.put(x),
                Err(_) => break,
            }
        }

        (input, Ok(c))
    }

    impl_parse_box! { Input }
}

impl<Input, P, F, C> ParserMut<Input> for ZeroOrMore<P, F>
where
    P: ParserMut<Input>,
    F: FnMut() -> C,
    C: Collection<P::Output>,
{
    #[inline]
    fn parse_mut(&mut self, mut input: Input) -> ParseResult<Input, Self> {
        let mut c = (self.1)();

        loop {
            let (next, out) = self.0.parse_mut(input);
            input = next;

            match out {
                Ok(x) => c.put(x),
                Err(_) => break,
            }
        }

        (input, Ok(c))
    }
}

impl<Input, P, F, C> Parser<Input> for ZeroOrMore<P, F>
where
    P: Parser<Input>,
    F: Fn() -> C,
    C: Collection<P::Output>,
{
    #[inline]
    fn parse(&self, mut input: Input) -> ParseResult<Input, Self> {
        let mut c = (self.1)();

        loop {
            let (next, out) = self.0.parse(input);
            input = next;

            match out {
                Ok(x) => c.put(x),
                Err(_) => break,
            }
        }

        (input, Ok(c))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct OneOrMore<P, F>(pub(crate) ZeroOrMore<P, F>);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FoundZero;

impl<Input: Restore, P, F, C> ParserOnce<Input> for OneOrMore<P, F>
where
    P: ParserMut<Input>,
    F: FnMut() -> C,
    C: Collection<P::Output>,
{
    type Output = C;
    type Error = FoundZero;

    #[inline]
    fn parse_once(self, input: Input) -> ParseResult<Input, Self> {
        let save = input.save();
        let (input, out) = self.0.parse_once(input);

        let out = out.unwrap();

        if out.is_empty() {
            (input.restore(save), Err(FoundZero))
        } else {
            (input, Ok(out))
        }
    }

    impl_parse_box! { Input }
}

impl<Input: Restore, P, F, C> ParserMut<Input> for OneOrMore<P, F>
where
    P: ParserMut<Input>,
    F: FnMut() -> C,
    C: Collection<P::Output>,
{
    #[inline]
    fn parse_mut(&mut self, input: Input) -> ParseResult<Input, Self> {
        let save = input.save();
        let (input, out) = self.0.parse_mut(input);

        let out = out.unwrap();

        if out.is_empty() {
            (input.restore(save), Err(FoundZero))
        } else {
            (input, Ok(out))
        }
    }
}

impl<Input: Restore, P, F, C> Parser<Input> for OneOrMore<P, F>
where
    P: Parser<Input>,
    F: Fn() -> C,
    C: Collection<P::Output>,
{
    #[inline]
    fn parse(&self, input: Input) -> ParseResult<Input, Self> {
        let save = input.save();
        let (input, out) = self.0.parse(input);

        let out = out.unwrap();

        if out.is_empty() {
            (input.restore(save), Err(FoundZero))
        } else {
            (input, Ok(out))
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Repeat<P, F, R>(pub(crate) P, pub(crate) F, pub(crate) R);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct RangeError;

impl<Input: Restore, P, F, R, C> ParserOnce<Input> for Repeat<P, F, R>
where
    P: ParserMut<Input>,
    R: RangeBounds<usize>,
    F: FnOnce() -> C,
    C: Collection<P::Output>,
{
    type Output = C;
    type Error = RangeError;

    #[inline]
    fn parse_once(mut self, mut input: Input) -> ParseResult<Input, Self> {
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

        let save = input.save();

        for _ in 1..min {
            let (next, out) = self.0.parse_mut(input);
            input = next;

            match out {
                Ok(x) => c.put(x),
                Err(_) => return (input.restore(save), Err(RangeError)),
            }
        }

        for _ in min..=max {
            let (next, out) = self.0.parse_mut(input);
            input = next;

            match out {
                Ok(x) => c.put(x),
                Err(_) => break,
            }
        }

        (input, Ok(c))
    }

    impl_parse_box! { Input }
}

impl<Input: Restore, P, F, R, C> ParserMut<Input> for Repeat<P, F, R>
where
    P: ParserMut<Input>,
    R: RangeBounds<usize>,
    F: FnMut() -> C,
    C: Collection<P::Output>,
{
    #[inline]
    fn parse_mut(&mut self, mut input: Input) -> ParseResult<Input, Self> {
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

        let save = input.save();

        for _ in 1..min {
            let (next, out) = self.0.parse_mut(input);
            input = next;

            match out {
                Ok(x) => c.put(x),
                Err(_) => return (input.restore(save), Err(RangeError)),
            }
        }

        for _ in min..=max {
            let (next, out) = self.0.parse_mut(input);
            input = next;

            match out {
                Ok(x) => c.put(x),
                Err(_) => break,
            }
        }

        (input, Ok(c))
    }
}

impl<Input: Restore, P, F, R, C> Parser<Input> for Repeat<P, F, R>
where
    P: Parser<Input>,
    R: RangeBounds<usize>,
    F: Fn() -> C,
    C: Collection<P::Output>,
{
    #[inline]
    fn parse(&self, mut input: Input) -> ParseResult<Input, Self> {
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

        let save = input.save();

        for _ in 1..min {
            let (next, out) = self.0.parse(input);
            input = next;

            match out {
                Ok(x) => c.put(x),
                Err(_) => return (input.restore(save), Err(RangeError)),
            }
        }

        for _ in min..=max {
            let (next, out) = self.0.parse(input);
            input = next;

            match out {
                Ok(x) => c.put(x),
                Err(_) => break,
            }
        }

        (input, Ok(c))
    }
}
