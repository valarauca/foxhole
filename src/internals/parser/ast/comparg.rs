use crate::internals::{
    canonization::graph::{
        build_typed_child_lambda, ChildLambda, Edge, EdgeTrait, Graph, Node, NodeIndex, NodeTrait,
    },
    parser::{
        ast::{ident::Ident, kind::Kind, op::Op, template::Template},
        span::{Span, Spanner},
    },
};
use serde::{Deserialize, Serialize};

stuff! {
    Name: CompositionalArg;
    Trait: CompositionalArgTrait;
    From: {
        Span => Primative => is_prim => get_prim,
        Template => Template => is_template => get_template,
        Ident => Func => is_func => get_func,
        Op => Op => is_op => get_op,
    }
}

/// Argument to a compositional function
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct CompositionalFunctionArg {
    pub arg: CompositionalArg,

    pub span: Box<Span>,
}

#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct CompFuncArgSpan;

impl EdgeTrait for CompFuncArgSpan {
    type N = Span;
}

#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct CompFuncArgPrim;

impl EdgeTrait for CompFuncArgPrim {
    type N = Span;
}

#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct CompFuncArgTemplate;

impl EdgeTrait for CompFuncArgTemplate {
    type N = Template;
}

#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct CompFuncArgIdent;

impl EdgeTrait for CompFuncArgIdent {
    type N = Ident;
}

#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct CompFuncArgOp;

impl EdgeTrait for CompFuncArgOp {
    type N = Op;
}

impl NodeTrait for CompositionalFunctionArg {
    fn children(&self) -> Vec<ChildLambda> {
        vec![
            build_typed_child_lambda::<_, CompFuncArgSpan>(&self.span),
            match &self.arg {
                CompositionalArg::Primative(ref prim) => {
                    build_typed_child_lambda::<_, CompFuncArgPrim>(prim)
                }
                CompositionalArg::Template(ref template) => {
                    build_typed_child_lambda::<_, CompFuncArgTemplate>(template)
                }
                CompositionalArg::Func(ref func) => {
                    build_typed_child_lambda::<_, CompFuncArgIdent>(func)
                }
                CompositionalArg::Op(ref op) => build_typed_child_lambda::<_, CompFuncArgOp>(op),
            },
        ]
    }
}

#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct CompositionalFunctionArgSpan;

impl EdgeTrait for CompositionalFunctionArgSpan {
    type N = Span;
}

impl CompositionalFunctionArg {
    pub(in crate::internals::parser) fn new<S, C>(
        arg: C,
        span: S,
    ) -> Result<Self, lrpar::Lexeme<u32>>
    where
        S: FnOnce() -> Result<Span, lrpar::Lexeme<u32>>,
        CompositionalArg: From<C>,
    {
        let span = Box::new(span()?);
        let arg = CompositionalArg::from(arg);
        Ok(Self { arg, span })
    }
}
impl AsRef<Span> for CompositionalFunctionArg {
    fn as_ref(&self) -> &Span {
        &self.span
    }
}
impl Spanner for CompositionalFunctionArg {}

/// Declaring a compositional function
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct CompositionalFunction {
    pub name: Box<Ident>,

    pub null_arg: Box<CompositionalFunctionArg>,

    pub single_arg: Box<CompositionalFunctionArg>,

    pub collection_arg: Box<CompositionalFunctionArg>,
    pub ret: Box<Kind>,

    pub span: Box<Span>,
}

#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct CompositionalFunctionName;

impl EdgeTrait for CompositionalFunctionName {
    type N = Ident;
}

#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct CompositionalFunctionReturnType;

impl EdgeTrait for CompositionalFunctionReturnType {
    type N = Kind;
}

#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct CompositionalFunctionSpan;

impl EdgeTrait for CompositionalFunctionSpan {
    type N = Span;
}

#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct CompositionalCollectionArg;

impl EdgeTrait for CompositionalCollectionArg {
    type N = CompositionalFunctionArg;
}

#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct CompositionalSingleArg;

impl EdgeTrait for CompositionalSingleArg {
    type N = CompositionalFunctionArg;
}

#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct CompositionalNullArg;

impl EdgeTrait for CompositionalNullArg {
    type N = CompositionalFunctionArg;
}

impl NodeTrait for CompositionalFunction {
    fn children(&self) -> Vec<ChildLambda> {
        vec![
            build_typed_child_lambda::<_, CompositionalFunctionName>(&self.name),
            build_typed_child_lambda::<_, CompositionalFunctionReturnType>(&self.ret),
            build_typed_child_lambda::<_, CompositionalFunctionSpan>(&self.span),
            build_typed_child_lambda::<_, CompositionalNullArg>(&self.null_arg),
            build_typed_child_lambda::<_, CompositionalSingleArg>(&self.single_arg),
            build_typed_child_lambda::<_, CompositionalCollectionArg>(&self.collection_arg),
        ]
    }
}

impl AsRef<Span> for CompositionalFunction {
    fn as_ref(&self) -> &Span {
        &self.span
    }
}
impl Spanner for CompositionalFunction {}
impl CompositionalFunction {
    pub(in crate::internals::parser) fn new<S>(
        name: Ident,
        n_arg: CompositionalFunctionArg,
        s_arg: CompositionalFunctionArg,
        c_arg: CompositionalFunctionArg,
        ret: Kind,
        span: S,
    ) -> Result<Self, lrpar::Lexeme<u32>>
    where
        S: FnOnce() -> Result<Span, lrpar::Lexeme<u32>>,
    {
        let span = Box::new(span()?);
        let name = Box::new(name);
        let null_arg = Box::new(n_arg);
        let single_arg = Box::new(s_arg);
        let collection_arg = Box::new(c_arg);
        let ret = Box::new(ret);
        Ok(Self {
            name,
            null_arg,
            single_arg,
            collection_arg,
            ret,
            span,
        })
    }
}
