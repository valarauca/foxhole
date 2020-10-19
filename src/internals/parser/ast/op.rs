use serde::{Deserialize, Serialize};

use crate::internals::parser::ast::expr::Expression;
use crate::internals::parser::span::{Span, Spanner};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Operation {
    pub left: Box<Expression>,

    pub op: Op,

    pub right: Box<Expression>,

    pub span: Box<Span>,
}

impl AsRef<Span> for Operation {
    fn as_ref(&self) -> &Span {
        &self.span
    }
}

impl Spanner for Operation {
    fn fields(&self) {
        self.set_id();
        self.left.fields();
        self.right.fields();
    }
}

impl Operation {
    pub(in crate::internals::parser) fn new<F>(
        left: Expression,
        op: Op,
        right: Expression,
        span: F,
    ) -> Result<Self, lrpar::Lexeme<u32>>
    where
        F: FnOnce() -> Result<Span, lrpar::Lexeme<u32>>,
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
