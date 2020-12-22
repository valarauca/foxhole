use crate::internals::{
    canonization::graph::{ChildLambda, Edge, EdgeTrait, Graph, Node, NodeIndex, NodeTrait, build_typed_child_lambda, build_data_child_lambda},
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
        let mut v = vec![
            build_typed_child_lambda::<_,FunctionDecReturnType>(&self.ret),
            build_typed_child_lambda::<_,FunctionDecSpan>(&self.span),
            build_typed_child_lambda::<_,FunctionDecName>(&self.name),
        ];
        v.extend(
            self.args.iter().enumerate().map(|(pos,arg)| build_data_child_lambda(arg,FunctionDecArgs(pos))));
        v.extend(
            self.body.iter().enumerate().map(|(pos,statement)| build_data_child_lambda(statement, FunctionDecStatement(pos))));
        v
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
