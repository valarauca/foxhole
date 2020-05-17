use crate::internals::parser::span::{Span, Spanner};
use crate::internals::parser::ast::ident::{Identifier};
use crate::internals::parser::ast::expr::{Expression};

#[derive(Clone, Debug)]
pub struct FunctionDeclaration<'input> {
    pub name: Identifier<'input>,
    pub args: Vec<Expression<'input>>,
    pub body: Vec<Expression<'input>>,
    pub span: Span<'input>,
}
impl<'input> AsRef<Span<'input>> for FunctionDeclaration<'input> {
    fn as_ref(&self) -> &Span<'input> {
        &self.span
    }
}
impl<'input> Spanner<'input> for FunctionDeclaration<'input> {}
