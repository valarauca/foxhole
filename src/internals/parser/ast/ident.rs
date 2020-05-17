
use crate::internals::parser::span::{Span, Spanner};

/// Identifier is a parsed identifier. A function name, a variable, etc.
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub struct Ident<'input> {
    span: Span<'input>,
}
impl<'input> Ident<'input> {

    /// constructs a new identifier from a span
    #[inline(always)]
    pub(in crate::internals::parser) fn new(span: Span<'input>) -> Self {
        Self { span }
    }
}
impl<'input> AsRef<Span<'input>> for Ident<'input> {
    fn as_ref(&self) -> &Span<'input> {
        &self.span
    }
}
impl<'input> Spanner<'input> for Ident<'input> {}
