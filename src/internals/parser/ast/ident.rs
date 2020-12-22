use serde::{Deserialize, Serialize};

use crate::internals::{
    canonization::graph::{ChildLambda, Edge, EdgeTrait, Graph, Node, NodeIndex, NodeTrait, build_data_child_lambda},
    parser::span::{Span, Spanner},
};

/// Identifier is a parsed identifier. A function name, a variable, etc.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Ident {
    span: Span,
}
impl Ident {
    /// constructs a new identifier from a span
    #[inline(always)]
    pub(in crate::internals::parser) fn new(span: Span) -> Self {
        Self { span }
    }
}
impl AsRef<Span> for Ident {
    fn as_ref(&self) -> &Span {
        &self.span
    }
}
impl Spanner for Ident {
}

#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct IdentSpanEdge;

impl EdgeTrait for IdentSpanEdge {
    type N = Span;
}

impl NodeTrait for Ident {
    fn children(&self) -> Vec<ChildLambda> {
        vec![build_data_child_lambda(&self.span,IdentSpanEdge::default())]
    }
}

impl NodeTrait for Span {}
