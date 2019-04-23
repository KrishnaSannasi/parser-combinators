use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct AndThen<P, F>(pub(crate) P, pub(crate) F);

impl<Input: Clone, P, F, Q> Parser<Input> for AndThen<P, F>
where P: Parser<Input>, 
      Q: Parser<Input>,
      F: FnMut(P::Output) -> Q {
    type Output = Q::Output;
    type Error = Either<P::Error, Q::Error>;

    fn parse(&mut self, input: Input) -> ParseResult<Input, Self> {
        let old_input = input.clone();
        let (input, out) = self.0.parse(input);
        
        match out {
            Err(x) => (old_input, Err(Either::Left(x))),
            Ok(out) => {
                let (input, out) = (self.1)(out).parse(input);

                match out {
                    Err(x) => (old_input, Err(Either::Right(x))),
                    Ok(out) => (input, Ok(out)),
                }
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct OrElse<P, Q>(pub(crate) P, pub(crate) Q);

impl<Input: Clone, P, F, Q> Parser<Input> for OrElse<P, F>
where P: Parser<Input>, 
      Q: Parser<Input>,
      F: FnMut(P::Error) -> Q {
    type Output = Either<P::Output, Q::Output>;
    type Error = Q::Error;

    fn parse(&mut self, input: Input) -> ParseResult<Input, Self> {
        let old_input = input.clone();
        let (input, out) = self.0.parse(input);
        
        match out {
            Ok(x) => (input, Ok(Either::Left(x))),
            Err(out) => {
                let (input, out) = (self.1)(out).parse(input);

                match out {
                    Ok(x) => (input, Ok(Either::Right(x))),
                    Err(out) => (old_input, Err(out)),
                }
            },
        }
    }
}
