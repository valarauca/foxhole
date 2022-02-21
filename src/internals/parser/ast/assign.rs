use crate::internals::{
    parser::{
        ast::{expr::Expression, ident::Ident, kind::Kind},
        span::{Span, Spanner},
    },
};
use serde::{Deserialize, Serialize};

/// Assign represents an assignment
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Assign {
    pub name: Box<Ident>,

    pub expr: Box<Expression>,
    pub kind: Box<Option<Kind>>,

    pub span: Box<Span>,
}

impl AsRef<Span> for Assign {
    fn as_ref(&self) -> &Span {
        &self.span
    }
}

impl Spanner for Assign {}

impl Assign {
    pub(in crate::internals::parser) fn new<F, K>(
        name: Ident,
        kind: K,
        expr: Expression,
        span: F,
    ) -> Result<Self, lrpar::Lexeme<u32>>
    where
        K: Into<Option<Kind>>,
        F: FnOnce() -> Result<Span, lrpar::Lexeme<u32>>,
    {
        let span = Box::new(span()?);
        let name = Box::new(name);
        let kind = Box::new(kind.into());
        let expr = Box::new(expr);
        Ok(Self {
            name,
            expr,
            kind,
            span,
        })
    }
}
