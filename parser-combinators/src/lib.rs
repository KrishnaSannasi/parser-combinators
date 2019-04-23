
#![feature(specialization)]

use std::convert::Infallible;

mod infallible;

use either::Either;

pub mod map;
pub mod flat_map;
pub mod then;
pub mod and_then;
pub mod inspect;
pub mod func;
pub mod filter;
pub mod repeat;

use map::*;
use flat_map::*;
use then::*;
use and_then::*;
use inspect::*;
use filter::*;
use repeat::*;

pub mod prelude {
    pub use crate::Parser;
    pub use crate::func::AsParser as _;
    
    pub use crate::reject;
    pub use crate::unimplemented_parser;

    pub fn defer<P, Input>(p: P) -> crate::func::Defer<P>
    where crate::func::Defer<P>: Parser<Input> {
        p.defer()
    }

    pub mod util {
        pub use crate::infallible::*;

        pub fn ignore() {}

        pub fn fst<T, U>((t, _): (T, U)) -> T { t }
        pub fn snd<T, U>((_, u): (T, U)) -> U { u }
    }

    #[macro_export]
    macro_rules! reject {
        ($($type:tt)*) => { <$crate::Reject as $crate::Parser<$($type)*>>::map($crate::Reject, $crate::prelude::util::IntoInfallible::into_infallible) };
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
type ParseResult<Input, P: Parser<Input>> = (Input, Result<P::Output, P::Error>);

pub struct Accept;
pub struct Reject;

pub trait Parser<Input> {
    type Output;
    type Error;

    fn parse(&mut self, input: Input) -> ParseResult<Input, Self>;

    fn map<F>(self, f: F) -> Map<Self, F> where Self: Sized , Map<Self, F>: Parser<Input> { Map(self, f) }

    fn map_err<F>(self, f: F) -> MapErr<Self, F> where Self: Sized , MapErr<Self, F>: Parser<Input> { MapErr(self, f) }

    fn map_both<F, G>(self, f: F, g: G) -> MapBoth<Self, F, G> where Self: Sized , MapBoth<Self, F, G>: Parser<Input> { MapBoth(self, f, g) }

    fn flat_map<F>(self, f: F) -> FlatMap<Self, F> where Self: Sized , FlatMap<Self, F>: Parser<Input> { FlatMap(self, f) }

    // fn flat_map_err<F>(self, f: F) -> FlatMapErr<Self, F> where Self: Sized , FlatMapErr<Self, F>: Parser<Input>{ FlatMapErr(self, f) }

    // fn flat_map_both<F, G>(self, f: F, g: G) -> FlatMapBoth<Self, F, G> where Self: Sized , FlatMapBoth<Self, F, G>: Parser<Input>{ FlatMapBoth(self, f, g) }

    fn then<P>(self, p: P) -> Then<Self, P> where Self: Sized , Then<Self, P>: Parser<Input> { Then(self, p) }

    fn or<P>(self, p: P) -> Or<Self, P> where Self: Sized , Or<Self, P>: Parser<Input> { Or(self, p) }

    fn and_then<F>(self, f: F) -> AndThen<Self, F> where Self: Sized , AndThen<Self, F>: Parser<Input> { AndThen(self, f) }

    fn or_else<F>(self, f: F) -> OrElse<Self, F> where Self: Sized , OrElse<Self, F>: Parser<Input> { OrElse(self, f) }

    fn inspect<F>(self, f: F) -> Inspect<Self, F> where Self: Sized , Inspect<Self, F>: Parser<Input> { Inspect(self, f) }

    fn inspect_input<F>(self, f: F) -> InspectInput<Self, F> where Self: Sized , InspectInput<Self, F>: Parser<Input> { InspectInput(self, f) }

    fn filter<F>(self, f: F) -> Filter<Self, F> where Self: Sized , Filter<Self, F>: Parser<Input> { Filter(self, f) }

    fn filter_input<F>(self, f: F) -> FilterInput<Self, F> where Self: Sized , FilterInput<Self, F>: Parser<Input> { FilterInput(self, f) }

    fn optional(self) -> Optional<Self> where Self: Sized , Optional<Self>: Parser<Input> { Optional(self) }

    fn zero_or_more<F>(self, f: F) -> ZeroOrMore<Self, F> where Self: Sized , ZeroOrMore<Self, F>: Parser<Input> { ZeroOrMore(self, f) }

    fn one_or_more<F>(self, f: F) -> OneOrMore<Self, F> where Self: Sized , OneOrMore<Self, F>: Parser<Input> { OneOrMore(ZeroOrMore(self, f)) }

    fn repeat<F, R>(self, r: R, f: F) -> Repeat<Self, F, R> where Self: Sized , Repeat<Self, F, R>: Parser<Input> { Repeat(self, f, r) }
}

impl<Input> Parser<Input> for Accept {
    type Output = ();
    type Error = Infallible;

    fn parse(&mut self, input: Input) -> ParseResult<Input, Self> {
        (input, Ok(()))
    }
}

impl<Input> Parser<Input> for Reject {
    type Output = Infallible;
    type Error = ();

    fn parse(&mut self, input: Input) -> ParseResult<Input, Self> {
        (input, Err(()))
    }
}

impl<Input, P: ?Sized + Parser<Input>> Parser<Input> for Box<P> {
    type Output = P::Output;
    type Error = P::Error;

    fn parse(&mut self, input: Input) -> ParseResult<Input, Self> {
        P::parse(self, input)
    }
}

impl<Input, P: ?Sized + Parser<Input>> Parser<Input> for &mut P {
    type Output = P::Output;
    type Error = P::Error;

    fn parse(&mut self, input: Input) -> ParseResult<Input, Self> {
        P::parse(self, input)
    }
}
