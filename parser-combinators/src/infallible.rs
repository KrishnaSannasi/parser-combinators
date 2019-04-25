use super::*;
use either::Either;

pub use std::convert::Infallible;

#[allow(clippy::inline_always)]
#[inline(always)]
pub fn from_infallible<T>(i: Infallible) -> T {
    match i {}
}

pub trait IntoInfallible {
    fn into_infallible(self) -> Infallible;
}

impl IntoInfallible for Infallible {
    #[allow(clippy::inline_always)]
    #[inline(always)]
    fn into_infallible(self) -> Infallible {
        self
    }
}

impl<T: IntoInfallible, U: IntoInfallible> IntoInfallible for Either<T, U> {
    #[allow(clippy::inline_always)]
    #[inline(always)]
    fn into_infallible(self) -> Infallible {
        match self {
            Either::Left(x) => x.into_infallible(),
            Either::Right(x) => x.into_infallible(),
        }
    }
}

pub fn unwrap_left<L, N: IntoInfallible>(e: Either<L, N>) -> L {
    match e {
        Either::Left(l) => l,
        Either::Right(r) => from_infallible(r.into_infallible())
    }
}

pub fn unwrap_right<R, N: IntoInfallible>(e: Either<N, R>) -> R {
    match e {
        Either::Left(l) => from_infallible(l.into_infallible()),
        Either::Right(r) => r
    }
}

impl<Input> ParserOnce<Input> for Infallible {
    type Output = Infallible;
    type Error = Infallible;

    #[inline]
    fn parse_once(self, _: Input) -> ParseResult<Input, Self> {
        from_infallible(self)
    }
    
    impl_parse_box! { Input }
}

impl<Input> ParserMut<Input> for Infallible {
    #[inline]
    fn parse_mut(&mut self, _: Input) -> ParseResult<Input, Self> {
        from_infallible(*self)
    }
}

impl<Input> Parser<Input> for Infallible {
    #[inline]
    fn parse(&self, _: Input) -> ParseResult<Input, Self> {
        from_infallible(*self)
    }
}
