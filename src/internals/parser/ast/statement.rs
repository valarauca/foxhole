use crate::internals::parser::ast::assign::Assign;
use crate::internals::parser::ast::expr::Expression;
use crate::internals::parser::span::{Span, Spanner};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Statement<'input> {
    pub sttm: Box<State<'input>>,
    pub span: Box<Span<'input>>,
}
impl<'input> AsRef<Span<'input>> for Statement<'input> {
    fn as_ref(&self) -> &Span<'input> {
        &self.span
    }
}
impl<'input> Spanner<'input> for Statement<'input> {}
impl<'input> Statement<'input> {
    pub(in crate::internals::parser) fn new<I, S>(
        item: I,
        span: S,
    ) -> Result<Self, lrpar::Lexeme<u32>>
    where
        S: FnOnce() -> Result<Span<'input>, lrpar::Lexeme<u32>>,
        State<'input>: From<I>,
    {
        let span = Box::new(span()?);
        let sttm = Box::new(State::from(item));
        Ok(Self { sttm, span })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum State<'input> {
    Declaration(Box<Assign<'input>>),
    Termination(Box<Expression<'input>>),
}
impl<'input> From<Expression<'input>> for State<'input> {
    #[inline(always)]
    fn from(arg: Expression<'input>) -> Self {
        Self::Termination(Box::new(arg))
    }
}
impl<'input> From<Assign<'input>> for State<'input> {
    #[inline(always)]
    fn from(arg: Assign<'input>) -> Self {
        Self::Declaration(Box::new(arg))
    }
}
