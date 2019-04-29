#![forbid(unsafe_code)]
#![feature(specialization, unsized_locals)]

use std::rc::Rc;
use std::sync::Arc;

use std::convert::Infallible;

use either::Either;

macro_rules! impl_parse_box {
    ($Input:ident) => {
        fn parse_box(self: Box<Self>, input: $Input) -> ParseResult<$Input, Self> {
            (*self).parse_once(input)
        }
    };
}

mod infallible;

pub mod and_then;
pub mod filter;
pub mod flat_map;
pub mod func;
pub mod inspect;
pub mod map;
pub mod repeat;
pub mod then;

use and_then::*;
use filter::*;
use flat_map::*;
use inspect::*;
use map::*;
use repeat::*;
use then::*;

pub mod prelude {
    pub use crate::func::AsParser as _;
    pub use crate::{Parser, ParserMut, ParserOnce};

    pub use crate::reject;
    pub use crate::unimplemented_parser;

    pub fn defer_once<P, Input>(p: P) -> crate::func::DeferOnce<P>
    where
        crate::func::DeferOnce<P>: ParserOnce<Input>,
    {
        p.defer_once()
    }

    pub fn defer_mut<P, Input>(p: P) -> crate::func::DeferMut<P>
    where
        crate::func::DeferMut<P>: ParserMut<Input>,
    {
        p.defer_mut()
    }

    pub fn defer<P, Input>(p: P) -> crate::func::Defer<P>
    where
        crate::func::Defer<P>: Parser<Input>,
    {
        p.defer()
    }

    pub mod util {
        pub use crate::infallible::*;

        pub fn ignore() {}

        pub fn count() -> crate::repeat::collections::Counter {
            crate::repeat::collections::Counter(0)
        }

        pub fn fst<T, U>((t, _): (T, U)) -> T {
            t
        }
        pub fn snd<T, U>((_, u): (T, U)) -> U {
            u
        }
    }

    #[macro_export]
    macro_rules! reject {
        ($($type:tt)*) => { <$crate::Reject as ParserCombinators<$($type)*>>::map($crate::Reject, $crate::prelude::util::IntoInfallible::into_infallible) };
    }

    #[macro_export]
    macro_rules! unimplemented_parser {
        ($($type:tt)*) => {{
            let x = unimplemented!();
            <$crate::prelude::util::Infallible as $crate::prelude::Parser<$($type)*>>::map_both(
                x,
                $crate::prelude::util::from_infallible,
                $crate::prelude::util::from_infallible
            )
        }};
    }
}

#[allow(type_alias_bounds)]
type ParseResult<Input, P: ParserMut<Input>> = (Input, Result<P::Output, P::Error>);

pub struct Accept;
pub struct Reject;

pub trait Restore {
    type SavePoint: Copy;

    fn save(&self) -> Self::SavePoint;
    fn restore(self, save: Self::SavePoint) -> Self;
}

impl<T: Copy> Restore for T {
    type SavePoint = Self;

    #[inline(always)]
    fn save(&self) -> Self {
        *self
    }

    #[inline(always)]
    fn restore(self, save: Self::SavePoint) -> Self {
        save
    }
}

pub trait Parser<Input>: ParserMut<Input> {
    fn parse(&self, input: Input) -> ParseResult<Input, Self>;
}

pub trait ParserMut<Input>: ParserOnce<Input> {
    fn parse_mut(&mut self, input: Input) -> ParseResult<Input, Self>;
}

pub trait ParseBox<Input>: ParserOnce<Input> {
    fn parse_box(self: Box<Self>, input: Input) -> ParseResult<Input, Self>;
}

pub trait ParserOnce<Input> {
    type Output;
    type Error;

    fn parse_once(self, input: Input) -> ParseResult<Input, Self>
    where
        Self: Sized;

    fn parse_box(self: Box<Self>, input: Input) -> ParseResult<Input, Self>;

    #[inline]
    fn map<F>(self, f: F) -> Map<Self, F>
    where
        Self: Sized,
    {
        Map(self, f)
    }

    #[inline]
    fn map_err<F>(self, f: F) -> MapErr<Self, F>
    where
        Self: Sized,
    {
        MapErr(self, f)
    }

    #[inline]
    fn map_both<F, G>(self, f: F, g: G) -> MapBoth<Self, F, G>
    where
        Self: Sized,
    {
        MapBoth(self, f, g)
    }

    #[inline]
    fn flat_map<F>(self, f: F) -> FlatMap<Self, F>
    where
        Self: Sized,
    {
        FlatMap(self, f)
    }

    #[inline]
    fn flat_map_err<F>(self, f: F) -> FlatMapErr<Self, F>
    where
        Self: Sized,
    {
        FlatMapErr(self, f)
    }

    #[inline]
    fn flat_map_both<F, G>(self, f: F, g: G) -> FlatMapBoth<Self, F, G>
    where
        Self: Sized,
    {
        FlatMapBoth(self, f, g)
    }

    #[inline]
    fn then<P>(self, p: P) -> Then<Self, P>
    where
        Self: Sized,
    {
        Then(self, p)
    }

    #[inline]
    fn or<P>(self, p: P) -> Or<Self, P>
    where
        Self: Sized,
    {
        Or(self, p)
    }

    #[inline]
    fn and_then<F>(self, f: F) -> AndThen<Self, F>
    where
        Self: Sized,
    {
        AndThen(self, f)
    }

    #[inline]
    fn or_else<F>(self, f: F) -> OrElse<Self, F>
    where
        Self: Sized,
    {
        OrElse(self, f)
    }

    #[inline]
    fn inspect<F>(self, f: F) -> Inspect<Self, F>
    where
        Self: Sized,
    {
        Inspect(self, f)
    }

    #[inline]
    fn inspect_input<F>(self, f: F) -> InspectInput<Self, F>
    where
        Self: Sized,
    {
        InspectInput(self, f)
    }

    #[inline]
    fn filter<F>(self, f: F) -> Filter<Self, F>
    where
        Self: Sized,
    {
        Filter(self, f)
    }

    #[inline]
    fn filter_input<F>(self, f: F) -> FilterInput<Self, F>
    where
        Self: Sized,
    {
        FilterInput(self, f)
    }

    #[inline]
    fn optional(self) -> Optional<Self>
    where
        Self: Sized,
    {
        Optional(self)
    }

    #[inline]
    fn zero_or_more<F>(self, f: F) -> ZeroOrMore<Self, F>
    where
        Self: Sized,
    {
        ZeroOrMore(self, f)
    }

    #[inline]
    fn one_or_more<F>(self, f: F) -> OneOrMore<Self, F>
    where
        Self: Sized,
    {
        OneOrMore(ZeroOrMore(self, f))
    }

    #[inline]
    fn repeat<F, R>(self, r: R, f: F) -> Repeat<Self, F, R>
    where
        Self: Sized,
    {
        Repeat(self, f, r)
    }
}

impl<Input> ParserOnce<Input> for Accept {
    type Output = ();
    type Error = Infallible;

    #[inline]
    fn parse_once(self, input: Input) -> ParseResult<Input, Self> {
        (input, Ok(()))
    }

    impl_parse_box! { Input }
}

impl<Input> ParserMut<Input> for Accept {
    #[inline]
    fn parse_mut(&mut self, input: Input) -> ParseResult<Input, Self> {
        (input, Ok(()))
    }
}

impl<Input> Parser<Input> for Accept {
    #[inline]
    fn parse(&self, input: Input) -> ParseResult<Input, Self> {
        (input, Ok(()))
    }
}

impl<Input> ParserOnce<Input> for Reject {
    type Output = Infallible;
    type Error = ();

    #[inline]
    fn parse_once(self, input: Input) -> ParseResult<Input, Self> {
        (input, Err(()))
    }

    impl_parse_box! { Input }
}

impl<Input> ParserMut<Input> for Reject {
    #[inline]
    fn parse_mut(&mut self, input: Input) -> ParseResult<Input, Self> {
        (input, Err(()))
    }
}

impl<Input> Parser<Input> for Reject {
    #[inline]
    fn parse(&self, input: Input) -> ParseResult<Input, Self> {
        (input, Err(()))
    }
}

impl<Input, P: ?Sized + ParserOnce<Input>> ParserOnce<Input> for Box<P> {
    type Output = P::Output;
    type Error = P::Error;

    #[inline]
    fn parse_once(self, input: Input) -> ParseResult<Input, Self> {
        P::parse_box(self, input)
    }

    impl_parse_box! { Input }
}

impl<Input, P: ?Sized + ParserMut<Input>> ParserMut<Input> for Box<P> {
    #[inline]
    fn parse_mut(&mut self, input: Input) -> ParseResult<Input, Self> {
        P::parse_mut(self, input)
    }
}

impl<Input, P: ?Sized + Parser<Input>> Parser<Input> for Box<P> {
    #[inline]
    fn parse(&self, input: Input) -> ParseResult<Input, Self> {
        P::parse(self, input)
    }
}

impl<Input, P: ?Sized + Parser<Input>> ParserOnce<Input> for Rc<P> {
    type Output = P::Output;
    type Error = P::Error;

    #[inline]
    fn parse_once(self, input: Input) -> ParseResult<Input, Self> {
        P::parse(&self, input)
    }

    impl_parse_box! { Input }
}

impl<Input, P: ?Sized + Parser<Input>> ParserMut<Input> for Rc<P> {
    #[inline]
    fn parse_mut(&mut self, input: Input) -> ParseResult<Input, Self> {
        P::parse(self, input)
    }
}

impl<Input, P: ?Sized + Parser<Input>> Parser<Input> for Rc<P> {
    #[inline]
    fn parse(&self, input: Input) -> ParseResult<Input, Self> {
        P::parse(self, input)
    }
}

impl<Input, P: ?Sized + Parser<Input>> ParserOnce<Input> for Arc<P> {
    type Output = P::Output;
    type Error = P::Error;

    #[inline]
    fn parse_once(self, input: Input) -> ParseResult<Input, Self> {
        P::parse(&self, input)
    }

    impl_parse_box! { Input }
}

impl<Input, P: ?Sized + Parser<Input>> ParserMut<Input> for Arc<P> {
    #[inline]
    fn parse_mut(&mut self, input: Input) -> ParseResult<Input, Self> {
        P::parse(self, input)
    }
}

impl<Input, P: ?Sized + Parser<Input>> Parser<Input> for Arc<P> {
    #[inline]
    fn parse(&self, input: Input) -> ParseResult<Input, Self> {
        P::parse(self, input)
    }
}

impl<Input, P: ?Sized + ParserMut<Input>> ParserOnce<Input> for &mut P {
    type Output = P::Output;
    type Error = P::Error;

    #[inline]
    fn parse_once(self, input: Input) -> ParseResult<Input, Self> {
        P::parse_mut(self, input)
    }

    impl_parse_box! { Input }
}

impl<Input, P: ?Sized + ParserMut<Input>> ParserMut<Input> for &mut P {
    #[inline]
    fn parse_mut(&mut self, input: Input) -> ParseResult<Input, Self> {
        P::parse_mut(self, input)
    }
}

impl<Input, P: ?Sized + Parser<Input>> Parser<Input> for &mut P {
    #[inline]
    fn parse(&self, input: Input) -> ParseResult<Input, Self> {
        P::parse(self, input)
    }
}

impl<Input, P: ?Sized + Parser<Input>> ParserOnce<Input> for &P {
    type Output = P::Output;
    type Error = P::Error;

    #[inline]
    fn parse_once(self, input: Input) -> ParseResult<Input, Self> {
        P::parse(self, input)
    }

    impl_parse_box! { Input }
}

impl<Input, P: ?Sized + Parser<Input>> ParserMut<Input> for &P {
    #[inline]
    fn parse_mut(&mut self, input: Input) -> ParseResult<Input, Self> {
        P::parse(self, input)
    }
}

impl<Input, P: ?Sized + Parser<Input>> Parser<Input> for &P {
    #[inline]
    fn parse(&self, input: Input) -> ParseResult<Input, Self> {
        P::parse(self, input)
    }
}
