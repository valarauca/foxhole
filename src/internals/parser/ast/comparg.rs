use crate::internals::{
    canonization::graph::{ChildLambda, Edge, EdgeTrait, Graph, Node, NodeIndex, NodeTrait},
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
        let span: Span = self.span.as_ref().clone();
        let mut v: Vec<ChildLambda> = vec![Box::new(move |graph, parent| {
            let id = graph.build_from_root(span);
            graph.add_edge(parent, id, CompFuncArgSpan::default());
        })];
        let lambda: ChildLambda = match &self.arg {
            CompositionalArg::Primative(ref prim) => {
                let span: Span = prim.as_ref().clone();
                Box::new(move |graph, parent| {
                    let id = graph.build_from_root(span);
                    graph.add_edge(parent, id, CompFuncArgPrim::default());
                })
            }
            CompositionalArg::Template(ref template) => {
                let template: Template = template.as_ref().clone();
                Box::new(move |graph, parent| {
                    let id = graph.build_from_root(template);
                    graph.add_edge(parent, id, CompFuncArgTemplate::default());
                })
            }
            CompositionalArg::Func(ref func) => {
                let ident: Ident = func.as_ref().clone();
                Box::new(move |graph, parent| {
                    let id = graph.build_from_root(ident);
                    graph.add_edge(parent, id, CompFuncArgIdent::default());
                })
            }
            CompositionalArg::Op(ref op) => {
                let op: Op = op.as_ref().clone();
                Box::new(move |graph, parent| {
                    let id = graph.build_from_root(op);
                    graph.add_edge(parent, id, CompFuncArgOp::default());
                })
            }
        };
        v.push(lambda);
        v
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
impl Spanner for CompositionalFunctionArg {
    fn fields(&self) {
        self.set_id();
        match &self.arg {
            CompositionalArg::Primative(ref a) => a.set_id(),
            CompositionalArg::Template(ref b) => b.fields(),
            CompositionalArg::Func(ref c) => c.fields(),
            _ => {}
        };
    }
}

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
        let name: Ident = self.name.as_ref().clone();
        let null_arg: CompositionalFunctionArg = self.null_arg.as_ref().clone();
        let single_arg: CompositionalFunctionArg = self.single_arg.as_ref().clone();
        let collection_arg: CompositionalFunctionArg = self.collection_arg.as_ref().clone();
        let ret: Kind = self.ret.as_ref().clone();
        let span: Span = self.span.as_ref().clone();
        vec![
            Box::new(move |graph, parent| {
                let id = graph.build_from_root(name);
                graph.add_edge(parent, id, CompositionalFunctionName::default());
            }),
            Box::new(move |graph, parent| {
                let id = graph.build_from_root(ret);
                graph.add_edge(parent, id, CompositionalFunctionReturnType::default());
            }),
            Box::new(move |graph, parent| {
                let id = graph.build_from_root(span);
                graph.add_edge(parent, id, CompositionalFunctionSpan::default());
            }),
            Box::new(move |graph, parent| {
                let id = graph.build_from_root(collection_arg);
                graph.add_edge(parent, id, CompositionalCollectionArg::default());
            }),
            Box::new(move |graph, parent| {
                let id = graph.build_from_root(single_arg);
                graph.add_edge(parent, id, CompositionalSingleArg::default());
            }),
            Box::new(move |graph, parent| {
                let id = graph.build_from_root(null_arg);
                graph.add_edge(parent, id, CompositionalNullArg::default());
            }),
        ]
    }
}

impl AsRef<Span> for CompositionalFunction {
    fn as_ref(&self) -> &Span {
        &self.span
    }
}
impl Spanner for CompositionalFunction {
    fn fields(&self) {
        self.set_id();
        self.name.fields();
        self.null_arg.fields();
        self.single_arg.fields();
        self.collection_arg.fields();
    }
}
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
