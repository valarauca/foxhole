use crate::internals::parser::ast::expr::Expression;
use crate::internals::parser::ast::ident::Ident;
use crate::internals::parser::span::{Span, Spanner};

/// Invoking a function
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Invoke<'input> {
    pub name: Ident<'input>,
    pub args: Box<[Expression<'input>]>,
    pub span: Span<'input>,
}
impl<'input> AsRef<Span<'input>> for Invoke<'input> {
    fn as_ref(&self) -> &Span<'input> {
        &self.span
    }
}
impl<'input> Spanner<'input> for Invoke<'input> {}

impl<'input> Invoke<'input> {
    pub fn new<I, F>(name: Ident<'input>, args: I, span: F) -> Result<Self, lrpar::Lexeme<u32>>
    where
        I: IntoIterator<Item = Expression<'input>>,
        F: FnOnce() -> Result<Span<'input>, lrpar::Lexeme<u32>>,
    {
        let span = span()?;
        let args = args
            .into_iter()
            .collect::<Vec<Expression<'input>>>()
            .into_boxed_slice();
        Ok(Self { name, args, span })
    }
}
