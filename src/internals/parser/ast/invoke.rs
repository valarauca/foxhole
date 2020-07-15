use serde::{Deserialize, Serialize};

use crate::internals::parser::ast::expr::Expression;
use crate::internals::parser::ast::ident::Ident;
use crate::internals::parser::span::{Span, Spanner};

/// Invoking a function
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Invoke<'input> {
    #[serde(borrow)]
    pub name: Box<Ident<'input>>,
    #[serde(borrow)]
    pub args: Box<[Expression<'input>]>,
    #[serde(borrow)]
    pub span: Box<Span<'input>>,
}
impl<'input> AsRef<Span<'input>> for Invoke<'input> {
    fn as_ref(&self) -> &Span<'input> {
        &self.span
    }
}
impl<'input> Spanner<'input> for Invoke<'input> {}

impl<'input> Invoke<'input> {
    pub(in crate::internals::parser) fn new<I, F>(
        name: Ident<'input>,
        args: I,
        span: F,
    ) -> Result<Self, lrpar::Lexeme<u32>>
    where
        I: IntoIterator<Item = Expression<'input>>,
        F: FnOnce() -> Result<Span<'input>, lrpar::Lexeme<u32>>,
    {
        let span = Box::new(span()?);
        let args = args
            .into_iter()
            .collect::<Vec<Expression<'input>>>()
            .into_boxed_slice();
        let name = Box::new(name);
        Ok(Self { name, args, span })
    }
}
