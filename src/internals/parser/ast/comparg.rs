use serde::{Deserialize, Serialize};

use crate::internals::parser::ast::ident::Ident;
use crate::internals::parser::ast::kind::Kind;
use crate::internals::parser::ast::op::Op;
use crate::internals::parser::ast::template::Template;
use crate::internals::parser::span::{Span, Spanner};

stuff! {
    Name: CompositionalArg;
    Trait: CompositionalArgTrait;
    Lifetime: 'input;
    From: {
        Span<'input> => Primative => is_prim => get_prim,
        Template<'input> => Template => is_template => get_template,
        Ident<'input> => Func => is_func => get_func,
        Op<'input> => Op => is_op => get_op,
    }
}

/// Argument to a compositional function
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct CompositionalFunctionArg<'input> {
    #[serde(borrow)]
    pub arg: CompositionalArg<'input>,
    #[serde(borrow)]
    pub span: Box<Span<'input>>,
}
impl<'input> CompositionalFunctionArg<'input> {
    pub(in crate::internals::parser) fn new<S, C>(
        arg: C,
        span: S,
    ) -> Result<Self, lrpar::Lexeme<u32>>
    where
        S: FnOnce() -> Result<Span<'input>, lrpar::Lexeme<u32>>,
        CompositionalArg<'input>: From<C>,
    {
        let span = Box::new(span()?);
        let arg = CompositionalArg::from(arg);
        Ok(Self { arg, span })
    }
}
impl<'input> AsRef<Span<'input>> for CompositionalFunctionArg<'input> {
    fn as_ref(&self) -> &Span<'input> {
        &self.span
    }
}
impl<'input> Spanner<'input> for CompositionalFunctionArg<'input> {
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
pub struct CompositionalFunction<'input> {
    #[serde(borrow)]
    pub name: Box<Ident<'input>>,
    #[serde(borrow)]
    pub null_arg: Box<CompositionalFunctionArg<'input>>,
    #[serde(borrow)]
    pub single_arg: Box<CompositionalFunctionArg<'input>>,
    #[serde(borrow)]
    pub collection_arg: Box<CompositionalFunctionArg<'input>>,
    pub ret: Box<Kind>,
    #[serde(borrow)]
    pub span: Box<Span<'input>>,
}
impl<'input> AsRef<Span<'input>> for CompositionalFunction<'input> {
    fn as_ref(&self) -> &Span<'input> {
        &self.span
    }
}
impl<'input> Spanner<'input> for CompositionalFunction<'input> {
    fn fields(&self) {
        self.set_id();
        self.name.fields();
        self.null_arg.fields();
        self.single_arg.fields();
        self.collection_arg.fields();
    }
}
impl<'input> CompositionalFunction<'input> {
    pub(in crate::internals::parser) fn new<S>(
        name: Ident<'input>,
        n_arg: CompositionalFunctionArg<'input>,
        s_arg: CompositionalFunctionArg<'input>,
        c_arg: CompositionalFunctionArg<'input>,
        ret: Kind,
        span: S,
    ) -> Result<Self, lrpar::Lexeme<u32>>
    where
        S: FnOnce() -> Result<Span<'input>, lrpar::Lexeme<u32>>,
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
