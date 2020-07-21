use serde::{Deserialize, Serialize};

use crate::internals::parser::ast::expr::Expression;
use crate::internals::parser::span::{Span, Spanner};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Operation<'input> {
    #[serde(borrow)]
    pub left: Box<Expression<'input>>,
    #[serde(borrow)]
    pub op: Op<'input>,
    #[serde(borrow)]
    pub right: Box<Expression<'input>>,
    #[serde(borrow)]
    pub span: Box<Span<'input>>,
}

impl<'input> AsRef<Span<'input>> for Operation<'input> {
    fn as_ref(&self) -> &Span<'input> {
        &self.span
    }
}

impl<'input> Spanner<'input> for Operation<'input> {
    fn fields(&self) {
        self.set_id();
        self.left.fields();
        self.right.fields();
    }
}

impl<'input> Operation<'input> {
    pub(in crate::internals::parser) fn new<F>(
        left: Expression<'input>,
        op: Op<'input>,
        right: Expression<'input>,
        span: F,
    ) -> Result<Self, lrpar::Lexeme<u32>>
    where
        F: FnOnce() -> Result<Span<'input>, lrpar::Lexeme<u32>>,
    {
        let span = Box::new(span()?);
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

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Op<'input> {
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
    #[doc(hidden)]
    #[serde(borrow)]
    __LOL(std::marker::PhantomData<&'input ()>),
}
