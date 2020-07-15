use serde::{Deserialize, Serialize};

use crate::internals::parser::ast::ident::Ident;
use crate::internals::parser::ast::kind::Kind;
use crate::internals::parser::span::{Span, Spanner};

/// Argument to a function
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct FunctionArg<'input> {
    #[serde(borrow)]
    pub name: Box<Ident<'input>>,
    pub kind: Box<Kind>,
    #[serde(borrow)]
    pub span: Box<Span<'input>>,
}
impl<'input> FunctionArg<'input> {
    #[inline(always)]
    pub(in crate::internals::parser) fn new<F>(
        name: Ident<'input>,
        kind: Kind,
        span: F,
    ) -> Result<Self, lrpar::Lexeme<u32>>
    where
        F: FnOnce() -> Result<Span<'input>, lrpar::Lexeme<u32>>,
    {
        let span = Box::new(span()?);
        let name = Box::new(name);
        let kind = Box::new(kind);
        Ok(Self { name, kind, span })
    }
}
impl<'input> AsRef<Span<'input>> for FunctionArg<'input> {
    fn as_ref(&self) -> &Span<'input> {
        &self.span
    }
}
impl<'input> Spanner<'input> for FunctionArg<'input> {}
