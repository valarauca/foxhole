use crate::internals::parser::ast::args::FunctionArg;
use crate::internals::parser::ast::ident::Ident;
use crate::internals::parser::ast::kind::Kind;
use crate::internals::parser::ast::statement::Statement;
use crate::internals::parser::span::{Span, Spanner};

/// Declaring a function
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FunctionDec<'input> {
    pub name: Box<Ident<'input>>,
    pub span: Box<Span<'input>>,
    pub args: Vec<FunctionArg<'input>>,
    pub body: Vec<Statement<'input>>,
    pub ret: Box<Kind>,
}
impl<'input> FunctionDec<'input> {
    #[inline(always)]
    pub(in crate::internals::parser) fn new<F, A, S>(
        name: Ident<'input>,
        args: A,
        body: S,
        ret: Kind,
        span: F,
    ) -> Result<Self, lrpar::Lexeme<u32>>
    where
        F: FnOnce() -> Result<Span<'input>, lrpar::Lexeme<u32>>,
        A: IntoIterator<Item = FunctionArg<'input>> + Sized,
        S: IntoIterator<Item = Statement<'input>> + Sized,
    {
        let span = Box::new(span()?);
        let args = args.into_iter().collect::<Vec<FunctionArg<'input>>>();
        let body = body.into_iter().collect::<Vec<Statement<'input>>>();
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
impl<'input> AsRef<Span<'input>> for FunctionDec<'input> {
    fn as_ref(&self) -> &Span<'input> {
        &self.span
    }
}
impl<'input> Spanner<'input> for FunctionDec<'input> {}