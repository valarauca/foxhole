use crate::internals::{
    canonization::graph::{ChildLambda, Edge, EdgeTrait, Graph, Node, NodeIndex, NodeTrait},
    parser::{
        ast::{ident::Ident, kind::Kind},
        span::{Span, Spanner},
    },
};
use serde::{Deserialize, Serialize};

/// Argument to a function
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct FunctionArg {
    pub name: Box<Ident>,
    pub kind: Box<Kind>,

    pub span: Box<Span>,
}

#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct FunctionArgIdent;

impl EdgeTrait for FunctionArgIdent {
    type N = Ident;
}

#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct FunctionArgKind;

impl EdgeTrait for FunctionArgKind {
    type N = Kind;
}

#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct FunctionArgSpan;

impl EdgeTrait for FunctionArgSpan {
    type N = Span;
}

impl NodeTrait for FunctionArg {
    fn children(&self) -> Vec<ChildLambda> {
        let name: Ident = self.name.as_ref().clone();
        let kind: Kind = self.kind.as_ref().clone();
        let span: Span = self.span.as_ref().clone();
        vec![
            Box::new(move |graph, parent| {
                let id = graph.build_from_root(name);
                graph.add_edge(parent, id, FunctionArgIdent);
            }),
            Box::new(move |graph, parent| {
                let id = graph.build_from_root(kind);
                graph.add_edge(parent, id, FunctionArgKind);
            }),
            Box::new(move |graph, parent| {
                let id = graph.build_from_root(span);
                graph.add_edge(parent, id, FunctionArgSpan);
            }),
        ]
    }
}

impl FunctionArg {
    #[inline(always)]
    pub(in crate::internals::parser) fn new<F>(
        name: Ident,
        kind: Kind,
        span: F,
    ) -> Result<Self, lrpar::Lexeme<u32>>
    where
        F: FnOnce() -> Result<Span, lrpar::Lexeme<u32>>,
    {
        let span = Box::new(span()?);
        let name = Box::new(name);
        let kind = Box::new(kind);
        Ok(Self { name, kind, span })
    }
}
impl AsRef<Span> for FunctionArg {
    fn as_ref(&self) -> &Span {
        &self.span
    }
}
impl Spanner for FunctionArg {
    fn fields(&self) {
        self.set_id();
        self.name.fields();
    }
}
