use crate::internals::parser::ast::expr::Expression;
use crate::internals::parser::ast::ident::Ident;
use crate::internals::parser::span::{Span, Spanner};

#[derive(Clone, Debug)]
pub struct Assign<'input> {
    pub name: Ident<'input>,
    pub expr: Expression<'input>,
    pub span: Span<'input>,
}
impl<'input> AsRef<Span<'input>> for Assign<'input> {
    fn as_ref(&self) -> &Span<'input> {
        &self.span
    }
}
impl<'input> Spanner<'input> for Assign<'input> {}

impl<'input> Assign<'input> {
    pub(in crate::internals::parser) fn new<F>(
        name: Ident<'input>,
        expr: Expression<'input>,
        span: F,
    ) -> Result<Self, lrpar::Lexeme<u32>>
    where
        F: FnOnce() -> Result<Span<'input>, lrpar::Lexeme<u32>>,
    {
        let span = span()?;
        Ok(Self { name, expr, span })
    }
}
