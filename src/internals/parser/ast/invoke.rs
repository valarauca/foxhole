use crate::internals::{
    canonization::graph::{ChildLambda, Edge, EdgeTrait, Graph, Node, NodeIndex, NodeTrait, build_typed_child_lambda, build_data_child_lambda},
    parser::{
        ast::{expr::Expression, ident::Ident},
        span::{Span, Spanner},
    },
};
use serde::{Deserialize, Serialize};

/// Invoking a function
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Invoke {
    pub name: Box<Ident>,

    pub args: Box<[Expression]>,

    pub span: Box<Span>,
}

#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct InvokeSpan;

impl EdgeTrait for InvokeSpan {
    type N = Span;
}

#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct InvokeIdent;

impl EdgeTrait for InvokeIdent {
    type N = Ident;
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct InvokeArg(usize);

impl EdgeTrait for InvokeArg {
    type N = Expression;
}

impl NodeTrait for Invoke {
    fn children(&self) -> Vec<ChildLambda> {
        let mut v = vec![
            build_typed_child_lambda::<_,InvokeSpan>(&self.span),
            build_typed_child_lambda::<_,InvokeIdent>(&self.name),
        ];
        v.extend(self.args.iter()
            .enumerate()
            .map(|(pos,expr)| build_data_child_lambda(expr,InvokeArg(pos))));
        v
    }
}

impl AsRef<Span> for Invoke {
    fn as_ref(&self) -> &Span {
        &self.span
    }
}
impl Spanner for Invoke {
}

impl Invoke {
    pub(in crate::internals::parser) fn new<I, F>(
        name: Ident,
        args: I,
        span: F,
    ) -> Result<Self, lrpar::Lexeme<u32>>
    where
        I: IntoIterator<Item = Expression>,
        F: FnOnce() -> Result<Span, lrpar::Lexeme<u32>>,
    {
        let span = Box::new(span()?);
        let args = args
            .into_iter()
            .collect::<Vec<Expression>>()
            .into_boxed_slice();
        let name = Box::new(name);
        Ok(Self { name, args, span })
    }
}
