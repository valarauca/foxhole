use serde::{Deserialize, Serialize};

use crate::internals::parser::span::{Span, Spanner};

/// Identifier is a parsed identifier. A function name, a variable, etc.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Ident {
    span: Span,
}
impl Ident {
    /// constructs a new identifier from a span
    #[inline(always)]
    pub(in crate::internals::parser) fn new(span: Span) -> Self {
        Self { span }
    }
}
impl AsRef<Span> for Ident {
    fn as_ref(&self) -> &Span {
        &self.span
    }
}
impl Spanner for Ident {
    fn fields(&self) {
        self.set_id();
    }
}
