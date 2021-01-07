use crate::internals::{
    canonization::graph::{
        build_typed_child_lambda, ChildLambda, Edge, EdgeTrait, Graph, Node, NodeIndex, NodeTrait,
    },
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
        vec![
            build_typed_child_lambda::<_, FunctionArgIdent>(&self.name),
            build_typed_child_lambda::<_, FunctionArgKind>(&self.kind),
            build_typed_child_lambda::<_, FunctionArgSpan>(&self.span),
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
impl Spanner for FunctionArg {}
