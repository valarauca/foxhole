use crate::internals::parser::ast::ident::Ident;
use crate::internals::parser::ast::kind::Kind;
use crate::internals::parser::ast::op::Op;
use crate::internals::parser::ast::template::Template;
use crate::internals::parser::span::{Span, Spanner};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CompositionalArg<'input> {
    Primative(Box<Span<'input>>),
    Template(Box<Template<'input>>),
    Func(Box<Ident<'input>>),
    Op(Box<Op>),
}
from_stuff! {
    'input;
    CompositionalArg<'input>;
    {
        Span<'input> => Primative,
        Template<'input> => Template,
        Ident<'input> => Func,
        Op => Op,
    }
}

/// Argument to a compositional function
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CompositionalFunctionArg<'input> {
    pub arg: CompositionalArg<'input>,
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
impl<'input> Spanner<'input> for CompositionalFunctionArg<'input> {}

/// Declaring a compositional function
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CompositionalFunction<'input> {
    pub name: Box<Ident<'input>>,
    pub null_arg: Box<CompositionalFunctionArg<'input>>,
    pub single_arg: Box<CompositionalFunctionArg<'input>>,
    pub collection_arg: Box<CompositionalFunctionArg<'input>>,
    pub ret: Box<Kind>,
    pub span: Box<Span<'input>>,
}
impl<'input> AsRef<Span<'input>> for CompositionalFunction<'input> {
    fn as_ref(&self) -> &Span<'input> {
        &self.span
    }
}
impl<'input> Spanner<'input> for CompositionalFunction<'input> {}
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
