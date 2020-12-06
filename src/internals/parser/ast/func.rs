use crate::internals::{
    canonization::graph::{ChildLambda, Edge, EdgeTrait, Graph, Node, NodeIndex, NodeTrait},
    parser::{
        ast::{args::FunctionArg, ident::Ident, kind::Kind, statement::Statement},
        span::{Span, Spanner},
    },
};
use serde::{Deserialize, Serialize};

/// Declaring a function
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct FunctionDec {
    pub name: Box<Ident>,

    pub span: Box<Span>,

    pub args: Vec<FunctionArg>,

    pub body: Vec<Statement>,
    pub ret: Box<Kind>,
}

#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct FunctionDecName;

impl EdgeTrait for FunctionDecName {
    type N = Ident;
}

#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct FunctionDecSpan;

impl EdgeTrait for FunctionDecSpan {
    type N = Span;
}

#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct FunctionDecArgs(usize);

impl EdgeTrait for FunctionDecArgs {
    type N = FunctionArg;
}

#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct FunctionDecStatement(usize);

impl EdgeTrait for FunctionDecStatement {
    type N = Statement;
}

#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct FunctionDecReturnType;

impl EdgeTrait for FunctionDecReturnType {
    type N = Kind;
}

impl NodeTrait for FunctionDec {
    fn children(&self) -> Vec<ChildLambda> {
        let arg_mapper = |(pos, arg): (usize, &FunctionArg)| -> ChildLambda {
            let arg = arg.clone();
            Box::new(move |graph, parent| {
                let id = graph.build_from_root(arg);
                graph.add_edge(parent, id, FunctionDecArgs(pos));
            })
        };
        let statement_mapper = |(pos, state): (usize, &Statement)| -> ChildLambda {
            let statement = state.clone();
            Box::new(move |graph, parent| {
                let id = graph.build_from_root(statement);
                graph.add_edge(parent, id, FunctionDecStatement(pos));
            })
        };

        let span: Span = self.span.as_ref().clone();
        let name: Ident = self.name.as_ref().clone();
        let kind: Kind = self.ret.as_ref().clone();
        let v: Vec<ChildLambda> = vec![
            Box::new(move |graph, parent| {
                let id = graph.build_from_root(name);
                graph.add_edge(parent, id, FunctionDecName::default());
            }),
            Box::new(move |graph, parent| {
                let id = graph.build_from_root(span);
                graph.add_edge(parent, id, FunctionDecSpan::default());
            }),
            Box::new(move |graph, parent| {
                let id = graph.build_from_root(kind);
                graph.add_edge(parent, id, FunctionDecReturnType::default());
            }),
        ];

        self.args
            .iter()
            .enumerate()
            .map(arg_mapper)
            .chain(self.body.iter().enumerate().map(statement_mapper))
            .chain(v.into_iter())
            .collect()
    }
}

impl FunctionDec {
    #[inline(always)]
    pub(in crate::internals::parser) fn new<F, A, S>(
        name: Ident,
        args: A,
        body: S,
        ret: Kind,
        span: F,
    ) -> Result<Self, lrpar::Lexeme<u32>>
    where
        F: FnOnce() -> Result<Span, lrpar::Lexeme<u32>>,
        A: IntoIterator<Item = FunctionArg> + Sized,
        S: IntoIterator<Item = Statement> + Sized,
    {
        let span = Box::new(span()?);
        let args = args.into_iter().collect::<Vec<FunctionArg>>();
        let body = body.into_iter().collect::<Vec<Statement>>();
        let name = Box::new(name);
        let ret = Box::new(ret);
        Ok(Self {
            name,
            span,
            args,
            body,
            ret,
        })
    }
}
impl AsRef<Span> for FunctionDec {
    fn as_ref(&self) -> &Span {
        &self.span
    }
}
impl Spanner for FunctionDec {
    fn fields(&self) {
        self.set_id();
        for arg in self.args.iter() {
            arg.fields();
        }
        for state in self.body.iter() {
            state.fields();
        }
    }
}
