use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct AndThen<P, F>(pub(crate) P, pub(crate) F);

impl<Input: Clone, P, F, Q> ParserOnce<Input> for AndThen<P, F>
where P: ParserOnce<Input>, 
      Q: ParserOnce<Input>,
      F: FnOnce(P::Output) -> Q {
    type Output = Q::Output;
    type Error = Either<P::Error, Q::Error>;
    
    fn parse_once(self, input: Input) -> ParseResult<Input, Self> {
        let old_input = input.clone();
        let (input, out) = self.0.parse_once(input);
        
        match out {
            Err(x) => (old_input, Err(Either::Left(x))),
            Ok(out) => {
                let (input, out) = (self.1)(out).parse_once(input);

                match out {
                    Err(x) => (old_input, Err(Either::Right(x))),
                    Ok(out) => (input, Ok(out)),
                }
            },
        }
    }
}

impl<Input: Clone, P, F, Q> ParserMut<Input> for AndThen<P, F>
where P: ParserMut<Input>, 
      Q: ParserOnce<Input>,
      F: FnMut(P::Output) -> Q {
    fn parse_mut(&mut self, input: Input) -> ParseResult<Input, Self> {
        let old_input = input.clone();
        let (input, out) = self.0.parse_mut(input);
        
        match out {
            Err(x) => (old_input, Err(Either::Left(x))),
            Ok(out) => {
                let (input, out) = (self.1)(out).parse_once(input);

                match out {
                    Err(x) => (old_input, Err(Either::Right(x))),
                    Ok(out) => (input, Ok(out)),
                }
            },
        }
    }
}

impl<Input: Clone, P, F, Q> Parser<Input> for AndThen<P, F>
where P: Parser<Input>, 
      Q: ParserOnce<Input>,
      F: Fn(P::Output) -> Q {
    fn parse(&self, input: Input) -> ParseResult<Input, Self> {
        let old_input = input.clone();
        let (input, out) = self.0.parse(input);
        
        match out {
            Err(x) => (old_input, Err(Either::Left(x))),
            Ok(out) => {
                let (input, out) = (self.1)(out).parse_once(input);

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

impl<Input: Clone, P, F, Q> ParserOnce<Input> for OrElse<P, F>
where P: ParserOnce<Input>, 
      Q: ParserOnce<Input>,
      F: FnOnce(P::Error) -> Q {
    type Output = Either<P::Output, Q::Output>;
    type Error = Q::Error;

    fn parse_once(self, input: Input) -> ParseResult<Input, Self> {
        let old_input = input.clone();
        let (input, out) = self.0.parse_once(input);
        
        match out {
            Ok(x) => (input, Ok(Either::Left(x))),
            Err(out) => {
                let (input, out) = (self.1)(out).parse_once(input);

                match out {
                    Ok(x) => (input, Ok(Either::Right(x))),
                    Err(out) => (old_input, Err(out)),
                }
            },
        }
    }
}

impl<Input: Clone, P, F, Q> ParserMut<Input> for OrElse<P, F>
where P: ParserMut<Input>, 
      Q: ParserOnce<Input>,
      F: FnMut(P::Error) -> Q {
    fn parse_mut(&mut self, input: Input) -> ParseResult<Input, Self> {
        let old_input = input.clone();
        let (input, out) = self.0.parse_mut(input);
        
        match out {
            Ok(x) => (input, Ok(Either::Left(x))),
            Err(out) => {
                let (input, out) = (self.1)(out).parse_once(input);

                match out {
                    Ok(x) => (input, Ok(Either::Right(x))),
                    Err(out) => (old_input, Err(out)),
                }
            },
        }
    }
}

impl<Input: Clone, P, F, Q> Parser<Input> for OrElse<P, F>
where P: Parser<Input>, 
      Q: ParserOnce<Input>,
      F: Fn(P::Error) -> Q {
    fn parse(&self, input: Input) -> ParseResult<Input, Self> {
        let old_input = input.clone();
        let (input, out) = self.0.parse(input);
        
        match out {
            Ok(x) => (input, Ok(Either::Left(x))),
            Err(out) => {
                let (input, out) = (self.1)(out).parse_once(input);

                match out {
                    Ok(x) => (input, Ok(Either::Right(x))),
                    Err(out) => (old_input, Err(out)),
                }
            },
        }
    }
}
