use crate::internals::{
    canonization::graph::{ChildLambda, Edge, EdgeTrait, Graph, Node, NodeIndex, NodeTrait, build_typed_child_lambda, build_data_child_lambda},
    parser::{
        ast::{expr::Expression, ident::Ident, kind::Kind},
        span::{Span, Spanner},
    },
};
use serde::{Deserialize, Serialize};

/// Assign represents an assignment
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Assign {
    pub name: Box<Ident>,

    pub expr: Box<Expression>,
    pub kind: Box<Option<Kind>>,

    pub span: Box<Span>,
}

#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct AssignIdent;

impl EdgeTrait for AssignIdent {
    type N = Ident;
}

#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct AssignKind;

impl EdgeTrait for AssignKind {
    type N = Kind;
}

#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct AssignExpr;

impl EdgeTrait for AssignExpr {
    type N = Expression;
}

#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct AssignSpan;

impl EdgeTrait for AssignSpan {
    type N = Span;
}

impl NodeTrait for Assign {
    fn children(&self) -> Vec<ChildLambda> {
        let mut v = vec![
            build_typed_child_lambda::<_,AssignSpan>(&self.span),
            build_typed_child_lambda::<_,AssignExpr>(&self.expr),
            build_typed_child_lambda::<_,AssignIdent>(&self.name),
        ];
        v.extend(self.kind.as_ref().clone().into_iter().map(|kind| build_data_child_lambda(&kind, AssignKind::default())));
        v
    }
}

impl AsRef<Span> for Assign {
    fn as_ref(&self) -> &Span {
        &self.span
    }
}

impl Spanner for Assign {
}

impl Assign {
    pub(in crate::internals::parser) fn new<F, K>(
        name: Ident,
        kind: K,
        expr: Expression,
        span: F,
    ) -> Result<Self, lrpar::Lexeme<u32>>
    where
        K: Into<Option<Kind>>,
        F: FnOnce() -> Result<Span, lrpar::Lexeme<u32>>,
    {
        let span = Box::new(span()?);
        let name = Box::new(name);
        let kind = Box::new(kind.into());
        let expr = Box::new(expr);
        Ok(Self {
            name,
            expr,
            kind,
            span,
        })
    }
}
