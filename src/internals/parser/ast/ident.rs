use serde::{Deserialize, Serialize};

use crate::internals::{
    parser::span::{Span, Spanner},
    canonization::graph::{
        EdgeTrait,NodeTrait,Graph,Node,Edge,NodeIndex,ChildLambda,
    }
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
    fn fields(&self) {
        self.set_id();
    }
}


#[derive(Default,Copy,Clone,PartialEq,Eq,PartialOrd,Ord)]
pub struct IdentSpanEdge;

impl EdgeTrait for IdentSpanEdge {
    type N = Span;
}

impl NodeTrait for Ident {

    fn children(&self) -> Vec<ChildLambda> {

        let arg = self.span.clone();
        let lambda = Box::new(move |graph: &mut Graph, parent: NodeIndex|{
            let id = graph.build_from_root(arg);
            graph.add_edge(parent,id,IdentSpanEdge);
        });

        vec![
            lambda
        ]
    }
}


impl NodeTrait for Span {
}

