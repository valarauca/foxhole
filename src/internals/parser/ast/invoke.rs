use crate::internals::{
    canonization::graph::{ChildLambda, Edge, EdgeTrait, Graph, Node, NodeIndex, NodeTrait},
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
        let name: Ident = self.name.as_ref().clone();

        let span: Span = self.span.as_ref().clone();

        let arg_mapper = |(pos, expr): (usize, &Expression)| -> ChildLambda {
            let expr = expr.clone();
            Box::new(move |graph, parent| {
                let id = graph.build_from_root(expr);
                graph.add_edge(parent, id, InvokeArg(pos));
            })
        };

        let mut output = self
            .args
            .iter()
            .enumerate()
            .map(arg_mapper)
            .collect::<Vec<ChildLambda>>();
        output.push(Box::new(move |graph, parent| {
            let id = graph.build_from_root(span);
            graph.add_edge(parent, id, InvokeSpan::default());
        }));
        output.push(Box::new(move |graph, parent| {
            let id = graph.build_from_root(name);
            graph.add_edge(parent, id, InvokeIdent::default());
        }));

        output
    }
}

impl AsRef<Span> for Invoke {
    fn as_ref(&self) -> &Span {
        &self.span
    }
}
impl Spanner for Invoke {
    fn fields(&self) {
        self.set_id();
        self.name.fields();
        for arg in self.args.as_ref().iter() {
            arg.fields();
        }
    }
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
