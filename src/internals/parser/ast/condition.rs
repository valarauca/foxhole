use serde::{Deserialize, Serialize};

use crate::internals::{
    canonization::graph::{
        build_typed_child_lambda, ChildLambda, Edge, EdgeTrait, Graph, Node, NodeIndex, NodeTrait,
    },
    parser::{
        ast::Expression,
        span::{Span, Spanner},
    },
};

/// Conditionals manage things like `if else`
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Conditional {
    pub condition: Box<Expression>,

    pub true_case: Box<Expression>,

    pub false_case: Box<Expression>,

    pub span: Box<Span>,
}

#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ConditionalCondition;

impl EdgeTrait for ConditionalCondition {
    type N = Expression;
}

#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ConditionalTrueCase;

impl EdgeTrait for ConditionalTrueCase {
    type N = Expression;
}

#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ConditionalFalseCase;

impl EdgeTrait for ConditionalFalseCase {
    type N = Expression;
}

#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ConditionalSpan;

impl EdgeTrait for ConditionalSpan {
    type N = Span;
}

impl NodeTrait for Conditional {
    fn children(&self) -> Vec<ChildLambda> {
        vec![
            build_typed_child_lambda::<_, ConditionalCondition>(&self.condition),
            build_typed_child_lambda::<_, ConditionalTrueCase>(&self.true_case),
            build_typed_child_lambda::<_, ConditionalFalseCase>(&self.false_case),
            build_typed_child_lambda::<_, ConditionalSpan>(&self.span),
        ]
    }
}

impl AsRef<Span> for Conditional {
    fn as_ref(&self) -> &Span {
        &self.span
    }
}
impl Spanner for Conditional {}

impl Conditional {
    pub(in crate::internals::parser) fn new<S>(
        condition: Expression,
        true_case: Expression,
        false_case: Expression,
        span: S,
    ) -> Result<Self, lrpar::Lexeme<u32>>
    where
        S: FnOnce() -> Result<Span, lrpar::Lexeme<u32>>,
    {
        let span = Box::new(span()?);
        let condition = Box::new(condition);
        let true_case = Box::new(true_case);
        let false_case = Box::new(false_case);
        Ok(Self {
            condition,
            true_case,
            false_case,
            span,
        })
    }
}
