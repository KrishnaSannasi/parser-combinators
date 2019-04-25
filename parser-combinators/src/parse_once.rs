use super::*;

impl<Input, P: ParserOnce<Input>> ParserCombinators<Input> for P {}

pub trait ParserCombinators<Input> {
    #[inline]
    fn map<F>(self, f: F) -> Map<Self, F>
    where
        Self: Sized,
        Map<Self, F>: ParserOnce<Input>,
    {
        Map(self, f)
    }

    #[inline]
    fn map_err<F>(self, f: F) -> MapErr<Self, F>
    where
        Self: Sized,
        MapErr<Self, F>: ParserOnce<Input>,
    {
        MapErr(self, f)
    }

    #[inline]
    fn map_both<F, G>(self, f: F, g: G) -> MapBoth<Self, F, G>
    where
        Self: Sized,
        MapBoth<Self, F, G>: ParserOnce<Input>,
    {
        MapBoth(self, f, g)
    }

    #[inline]
    fn flat_map<F>(self, f: F) -> FlatMap<Self, F>
    where
        Self: Sized,
        FlatMap<Self, F>: ParserOnce<Input>,
    {
        FlatMap(self, f)
    }

    #[inline]
    fn flat_map_err<F>(self, f: F) -> FlatMapErr<Self, F>
    where
        Self: Sized,
        FlatMapErr<Self, F>: ParserOnce<Input>,
    {
        FlatMapErr(self, f)
    }

    #[inline]
    fn flat_map_both<F, G>(self, f: F, g: G) -> FlatMapBoth<Self, F, G>
    where
        Self: Sized,
        FlatMapBoth<Self, F, G>: ParserOnce<Input>,
    {
        FlatMapBoth(self, f, g)
    }

    #[inline]
    fn then<P>(self, p: P) -> Then<Self, P>
    where
        Self: Sized,
        Then<Self, P>: ParserOnce<Input>,
    {
        Then(self, p)
    }

    #[inline]
    fn or<P>(self, p: P) -> Or<Self, P>
    where
        Self: Sized,
        Or<Self, P>: ParserOnce<Input>,
    {
        Or(self, p)
    }

    #[inline]
    fn and_then<F>(self, f: F) -> AndThen<Self, F>
    where
        Self: Sized,
        AndThen<Self, F>: ParserOnce<Input>,
    {
        AndThen(self, f)
    }

    #[inline]
    fn or_else<F>(self, f: F) -> OrElse<Self, F>
    where
        Self: Sized,
        OrElse<Self, F>: ParserOnce<Input>,
    {
        OrElse(self, f)
    }

    #[inline]
    fn inspect<F>(self, f: F) -> Inspect<Self, F>
    where
        Self: Sized,
        Inspect<Self, F>: ParserOnce<Input>,
    {
        Inspect(self, f)
    }

    #[inline]
    fn inspect_input<F>(self, f: F) -> InspectInput<Self, F>
    where
        Self: Sized,
        InspectInput<Self, F>: ParserOnce<Input>,
    {
        InspectInput(self, f)
    }

    #[inline]
    fn filter<F>(self, f: F) -> Filter<Self, F>
    where
        Self: Sized,
        Filter<Self, F>: ParserOnce<Input>,
    {
        Filter(self, f)
    }

    #[inline]
    fn filter_input<F>(self, f: F) -> FilterInput<Self, F>
    where
        Self: Sized,
        FilterInput<Self, F>: ParserOnce<Input>,
    {
        FilterInput(self, f)
    }

    #[inline]
    fn optional(self) -> Optional<Self>
    where
        Self: Sized,
        Optional<Self>: ParserOnce<Input>,
    {
        Optional(self)
    }

    #[inline]
    fn zero_or_more<F>(self, f: F) -> ZeroOrMore<Self, F>
    where
        Self: Sized,
        ZeroOrMore<Self, F>: ParserOnce<Input>,
    {
        ZeroOrMore(self, f)
    }

    #[inline]
    fn one_or_more<F>(self, f: F) -> OneOrMore<Self, F>
    where
        Self: Sized,
        OneOrMore<Self, F>: ParserOnce<Input>,
    {
        OneOrMore(ZeroOrMore(self, f))
    }

    #[inline]
    fn repeat<F, R>(self, r: R, f: F) -> Repeat<Self, F, R>
    where
        Self: Sized,
        Repeat<Self, F, R>: ParserOnce<Input>,
    {
        Repeat(self, f, r)
    }
}
