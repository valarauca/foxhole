use crate::internals::{
    parser::{
        ast::{ident::Ident, kind::Kind},
        span::{Span, Spanner},
    },
};
use serde::{Deserialize, Serialize};

/// Argument to a function
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct FunctionArg {
    pub name: Box<Ident>,
    pub kind: Box<Kind>,

    pub span: Box<Span>,
}

impl FunctionArg {
    #[inline(always)]
    pub(in crate::internals::parser) fn new<F>(
        name: Ident,
        kind: Kind,
        span: F,
    ) -> Result<Self, lrpar::Lexeme<u32>>
    where
        F: FnOnce() -> Result<Span, lrpar::Lexeme<u32>>,
    {
        let span = Box::new(span()?);
        let name = Box::new(name);
        let kind = Box::new(kind);
        Ok(Self { name, kind, span })
    }
}
impl AsRef<Span> for FunctionArg {
    fn as_ref(&self) -> &Span {
        &self.span
    }
}
impl Spanner for FunctionArg {}
