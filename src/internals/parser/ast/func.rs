use crate::internals::{
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
impl Spanner for FunctionDec {}
