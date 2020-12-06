use crate::internals::{
    canonization::graph::{ChildLambda, Edge, EdgeTrait, Graph, Node, NodeIndex, NodeTrait},
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
        let name: Ident = self.name.as_ref().clone();
        let expr: Expression = self.expr.as_ref().clone();
        let span: Span = self.span.as_ref().clone();
        let mut v: Vec<ChildLambda> = vec![
            Box::new(move |graph, parent| {
                let id = graph.build_from_root(name);
                graph.add_edge(parent, id, AssignIdent::default());
            }),
            Box::new(move |graph, parent| {
                let id = graph.build_from_root(expr);
                graph.add_edge(parent, id, AssignExpr::default());
            }),
            Box::new(move |graph, parent| {
                let id = graph.build_from_root(span);
                graph.add_edge(parent, id, AssignSpan::default());
            }),
        ];
        match self.kind.as_ref() {
            &Option::None => {}
            &Option::Some(ref kind) => {
                let kind: Kind = kind.clone();
                v.push(Box::new(move |graph, parent| {
                    let id = graph.build_from_root(kind);
                    graph.add_edge(parent, id, AssignKind::default());
                }))
            }
        };
        v
    }
}

impl AsRef<Span> for Assign {
    fn as_ref(&self) -> &Span {
        &self.span
    }
}

impl Spanner for Assign {
    fn fields(&self) {
        self.set_id();
        self.name.fields();
        self.expr.fields();
    }
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
