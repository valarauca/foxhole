use crate::internals::parser::ast::expr::Expression;
use crate::internals::parser::ast::ident::Ident;
use crate::internals::parser::ast::kind::Kind;
use crate::internals::parser::span::{Span, Spanner};

/// Assign represents an assignment
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Assign<'input> {
    pub name: Box<Ident<'input>>,
    pub expr: Box<Expression<'input>>,
    pub kind: Box<Option<Kind>>,
    pub span: Box<Span<'input>>,
}
impl<'input> AsRef<Span<'input>> for Assign<'input> {
    fn as_ref(&self) -> &Span<'input> {
        &self.span
    }
}
impl<'input> Spanner<'input> for Assign<'input> {}

impl<'input> Assign<'input> {
    pub(in crate::internals::parser) fn new<F, K>(
        name: Ident<'input>,
        kind: K,
        expr: Expression<'input>,
        span: F,
    ) -> Result<Self, lrpar::Lexeme<u32>>
    where
        K: Into<Option<Kind>>,
        F: FnOnce() -> Result<Span<'input>, lrpar::Lexeme<u32>>,
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
