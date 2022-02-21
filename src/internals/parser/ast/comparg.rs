use crate::internals::{
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
