use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FlatMap<P, F>(pub(crate) P, pub(crate) F);

impl<Input, P: Parser<Input>, F, Output> Parser<Input> for FlatMap<P, F>
where F: FnMut(P::Output) -> Result<Output, P::Error> {
    type Output = Output;
    type Error = P::Error;

    fn parse(&mut self, input: Input) -> ParseResult<Input, Self> {
        let (input, out) = self.0.parse(input);
        (input, out.and_then(&mut self.1))
    }
}

// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
// pub struct FlatMapErr<P, F>(pub(crate) P, pub(crate) F);

// impl<Input, P: Parser<Input>, F, Error> Parser<Input> for FlatMapErr<P, F>
// where F: FnMut(P::Error) -> Result<P::Output, Error> {
//     type Output = P::Output;
//     type Error = Error;

//     fn parse(&mut self, input: Input) -> ParseResult<Input, Self> {
//         let (input, out) = self.0.parse(input);
//         (input, out.or_else(&mut self.1))
//     }
// }

// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
// pub struct FlatMapBoth<P, F, G>(pub(crate) P, pub(crate) F, pub(crate) G);

// impl<Input, P: Parser<Input>, F, G, Output, Error> Parser<Input> for FlatMapBoth<P, F, G>
// where F: FnMut(P::Output) -> Result<Output, Error>,
//       G: FnMut(P::Error) -> Result<Output, Error> {
//     type Output = Output;
//     type Error = Error;

//     fn parse(&mut self, input: Input) -> ParseResult<Input, Self> {
//         let (input, out) = self.0.parse(input);
        
//         (
//             input, 
//             match out {
//                 Ok(x) => (self.1)(x),
//                 Err(x) => (self.2)(x)
//             }
//         )
//     }
// }
