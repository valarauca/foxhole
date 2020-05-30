use crate::internals::parser::ast::expr::Expression;
use crate::internals::parser::span::{Span, Spanner};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Operation<'input> {
    pub left: Box<Expression<'input>>,
    pub op: Op,
    pub right: Box<Expression<'input>>,
    pub span: Span<'input>,
}
impl<'input> AsRef<Span<'input>> for Operation<'input> {
    fn as_ref(&self) -> &Span<'input> {
        &self.span
    }
}
impl<'input> Spanner<'input> for Operation<'input> {}
impl<'input> Operation<'input> {
    pub(in crate::internals::parser) fn new<F>(
        left: Expression<'input>,
        op: Op,
        right: Expression<'input>,
        span: F,
    ) -> Result<Self, lrpar::Lexeme<u32>>
    where
        F: FnOnce() -> Result<Span<'input>, lrpar::Lexeme<u32>>,
    {
        let span = span()?;
        let left = Box::new(left);
        let right = Box::new(right);
        Ok(Self {
            left,
            op,
            right,
            span,
        })
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Op {
    ADD,
    SUB,
    MUL,
    DIV,
    EQ,
    NE,
    GT,
    LT,
    GE,
    LE,
    AND,
    OR,
    XOR,
}
