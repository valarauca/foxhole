use serde::{Deserialize, Serialize};

use crate::internals::{
    canonization::graph::{ChildLambda, Edge, EdgeTrait, Graph, Node, NodeIndex, NodeTrait, build_typed_child_lambda, build_data_child_lambda},
    parser::{
        ast::Expression,
        span::{Span, Spanner},
    },
};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Operation {
    pub left: Box<Expression>,

    pub op: Op,

    pub right: Box<Expression>,

    pub span: Box<Span>,
}

#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct OperationRight;

impl EdgeTrait for OperationRight {
    type N = Expression;
}

#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct OperationLeft;

impl EdgeTrait for OperationLeft {
    type N = Expression;
}

#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct OperationOp;

impl EdgeTrait for OperationOp {
    type N = Op;
}

#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct OperationSpan;

impl EdgeTrait for OperationSpan {
    type N = Span;
}

impl NodeTrait for Operation {
    fn children(&self) -> Vec<ChildLambda> {
        vec![
            build_typed_child_lambda::<_,OperationRight>(&self.left),
            build_typed_child_lambda::<_,OperationRight>(&self.right),
            build_typed_child_lambda::<_,OperationSpan>(&self.span),
            build_data_child_lambda(&self.op, OperationOp::default()),
        ]
    }
}

impl AsRef<Span> for Operation {
    fn as_ref(&self) -> &Span {
        &self.span
    }
}

impl Spanner for Operation {
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

impl NodeTrait for Op {}
