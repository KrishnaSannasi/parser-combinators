use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Then<P, Q>(pub(crate) P, pub(crate) Q);

impl<Input: Clone, P, Q> ParserOnce<Input> for Then<P, Q>
where
    P: ParserOnce<Input>,
    Q: ParserOnce<Input>,
{
    type Output = (P::Output, Q::Output);
    type Error = Either<P::Error, Q::Error>;

    #[inline]
    fn parse_once(self, input: Input) -> ParseResult<Input, Self> {
        let old_input = input.clone();
        let (input, out_0) = self.0.parse_once(input);

        match out_0 {
            Err(x) => (old_input, Err(Either::Left(x))),
            Ok(out_0) => {
                let (input, out_1) = self.1.parse_once(input);

                match out_1 {
                    Err(x) => (old_input, Err(Either::Right(x))),
                    Ok(out_1) => (input, Ok((out_0, out_1))),
                }
            }
        }
    }

    impl_parse_box! { Input }
}

impl<Input: Clone, P, Q> ParserMut<Input> for Then<P, Q>
where
    P: ParserMut<Input>,
    Q: ParserMut<Input>,
{
    #[inline]
    fn parse_mut(&mut self, input: Input) -> ParseResult<Input, Self> {
        let old_input = input.clone();
        let (input, out_0) = self.0.parse_mut(input);

        match out_0 {
            Err(x) => (old_input, Err(Either::Left(x))),
            Ok(out_0) => {
                let (input, out_1) = self.1.parse_mut(input);

                match out_1 {
                    Err(x) => (old_input, Err(Either::Right(x))),
                    Ok(out_1) => (input, Ok((out_0, out_1))),
                }
            }
        }
    }
}

impl<Input: Clone, P, Q> Parser<Input> for Then<P, Q>
where
    P: Parser<Input>,
    Q: Parser<Input>,
{
    #[inline]
    fn parse(&self, input: Input) -> ParseResult<Input, Self> {
        let old_input = input.clone();
        let (input, out_0) = self.0.parse(input);

        match out_0 {
            Err(x) => (old_input, Err(Either::Left(x))),
            Ok(out_0) => {
                let (input, out_1) = self.1.parse(input);

                match out_1 {
                    Err(x) => (old_input, Err(Either::Right(x))),
                    Ok(out_1) => (input, Ok((out_0, out_1))),
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Or<P, Q>(pub(crate) P, pub(crate) Q);

impl<Input: Clone, P, Q> ParserOnce<Input> for Or<P, Q>
where
    P: ParserOnce<Input>,
    Q: ParserOnce<Input>,
{
    type Output = Either<P::Output, Q::Output>;
    type Error = (P::Error, Q::Error);

    #[inline]
    default fn parse_once(self, input: Input) -> ParseResult<Input, Self> {
        let old_input = input.clone();
        let (input, out_0) = self.0.parse_once(input);

        match out_0 {
            Ok(x) => (input, Ok(Either::Left(x))),
            Err(out_0) => {
                let (input, out_1) = self.1.parse_once(input);

                match out_1 {
                    Ok(x) => (input, Ok(Either::Right(x))),
                    Err(out_1) => (old_input, Err((out_0, out_1))),
                }
            }
        }
    }

    impl_parse_box! { Input }
}

impl<Input: Clone, P, Q> ParserMut<Input> for Or<P, Q>
where
    P: ParserMut<Input>,
    Q: ParserMut<Input>,
{
    #[inline]
    default fn parse_mut(&mut self, input: Input) -> ParseResult<Input, Self> {
        let old_input = input.clone();
        let (input, out_0) = self.0.parse_mut(input);

        match out_0 {
            Ok(x) => (input, Ok(Either::Left(x))),
            Err(out_0) => {
                let (input, out_1) = self.1.parse_mut(input);

                match out_1 {
                    Ok(x) => (input, Ok(Either::Right(x))),
                    Err(out_1) => (old_input, Err((out_0, out_1))),
                }
            }
        }
    }
}

impl<Input: Clone, P, Q> Parser<Input> for Or<P, Q>
where
    P: Parser<Input>,
    Q: Parser<Input>,
{
    #[inline]
    default fn parse(&self, input: Input) -> ParseResult<Input, Self> {
        let old_input = input.clone();
        let (input, out_0) = self.0.parse(input);

        match out_0 {
            Ok(x) => (input, Ok(Either::Left(x))),
            Err(out_0) => {
                let (input, out_1) = self.1.parse(input);

                match out_1 {
                    Ok(x) => (input, Ok(Either::Right(x))),
                    Err(out_1) => (old_input, Err((out_0, out_1))),
                }
            }
        }
    }
}

impl<Input: Send + Clone, P, Q> ParserOnce<Input> for Or<P, Q>
where
    P: ParserOnce<Input> + Send,
    Q: ParserOnce<Input> + Send,

    P::Output: Send,
    P::Error: Send,
    Q::Output: Send,
    Q::Error: Send,
{
    #[inline]
    fn parse_once(self, input: Input) -> ParseResult<Input, Self> {
        let (old_input, input_0, input_1) = (input.clone(), input.clone(), input);
        let Or(first, second) = self;

        let ((input_0, out_0), (input_1, out_1)) =
            rayon::join(|| first.parse_once(input_0), || second.parse_once(input_1));

        match (out_0, out_1) {
            (Ok(out_0), _) => (input_0, Ok(Either::Left(out_0))),
            (_, Ok(out_1)) => (input_1, Ok(Either::Right(out_1))),
            (Err(err_0), Err(err_1)) => (old_input, Err((err_0, err_1))),
        }
    }
}

impl<Input: Send + Clone, P, Q> ParserMut<Input> for Or<P, Q>
where
    P: ParserMut<Input> + Send,
    Q: ParserMut<Input> + Send,

    P::Output: Send,
    P::Error: Send,
    Q::Output: Send,
    Q::Error: Send,
{
    #[inline]
    fn parse_mut(&mut self, input: Input) -> ParseResult<Input, Self> {
        let (old_input, input_0, input_1) = (input.clone(), input.clone(), input);
        let Or(first, second) = self;

        let ((input_0, out_0), (input_1, out_1)) =
            rayon::join(|| first.parse_mut(input_0), || second.parse_mut(input_1));

        match (out_0, out_1) {
            (Ok(out_0), _) => (input_0, Ok(Either::Left(out_0))),
            (_, Ok(out_1)) => (input_1, Ok(Either::Right(out_1))),
            (Err(err_0), Err(err_1)) => (old_input, Err((err_0, err_1))),
        }
    }
}

impl<Input: Send + Clone, P, Q> Parser<Input> for Or<P, Q>
where
    P: Parser<Input> + Sync,
    Q: Parser<Input> + Sync,

    P::Output: Send,
    P::Error: Send,
    Q::Output: Send,
    Q::Error: Send,
{
    #[inline]
    fn parse(&self, input: Input) -> ParseResult<Input, Self> {
        let (old_input, input_0, input_1) = (input.clone(), input.clone(), input);
        let Or(first, second) = self;

        let ((input_0, out_0), (input_1, out_1)) =
            rayon::join(move || first.parse(input_0), move || second.parse(input_1));

        match (out_0, out_1) {
            (Ok(out_0), _) => (input_0, Ok(Either::Left(out_0))),
            (_, Ok(out_1)) => (input_1, Ok(Either::Right(out_1))),
            (Err(err_0), Err(err_1)) => (old_input, Err((err_0, err_1))),
        }
    }
}
