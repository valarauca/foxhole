/*use crate::internals::parser::span::{Span, Spanner};
use crate::internals::parser::ast::ident::{Identifier};
use crate::internals::parser::ast::template::{Template};
use crate::internals::parser::ast::compfunc::CompositionalFunction;
use crate::tpp::ast::expr::Expression;
use crate::tpp::ast::func::FunctionDeclaration;*/

use crate::internals::parser::{
    span::{Span,Spanner},
    ast::{
        ident::{Identifier},
        template::{Template},
        compfunc::{CompositionalFunction},
        expr::{Expression},
        func::{FunctionDeclaration},
    }
};

#[derive(Clone, Debug)]
pub enum SourceStructure<'input> {
    CompFuncDecl(Box<CompositionalFunction<'input>>),
    FuncDecl(Box<FunctionDeclaration<'input>>),
    Expr(Box<Expression<'input>>),
    Comment(Box<Span<'input>>),
}
impl<'a> From<Span<'a>> for SourceStructure<'a> {
    fn from(x: Span<'a>) -> SourceStructure<'a> {
        SourceStructure::Comment(Box::new(x))
    }
}
impl<'a> From<CompositionalFunction<'a>> for SourceStructure<'a> {
    fn from(x: CompositionalFunction<'a>) -> SourceStructure<'a> {
        SourceStructure::CompFuncDecl(Box::new(x))
    }
}
impl<'a> From<FunctionDeclaration<'a>> for SourceStructure<'a> {
    fn from(x: FunctionDeclaration<'a>) -> SourceStructure<'a> {
        SourceStructure::FuncDecl(Box::new(x))
    }
}
impl<'a> From<Expression<'a>> for SourceStructure<'a> {
    fn from(x: Expression<'a>) -> SourceStructure<'a> {
        SourceStructure::Expr(Box::new(x))
    }
}

/// Source represents the full abstract syntax tree of
/// a file.
#[derive(Clone, Debug)]
pub struct Source<'input> {
    marker: std::marker::PhantomData<&'input ()>,
    pub data: Vec<SourceStructure<'input>>,
}
impl<'input> Default for Source<'input> {
    fn default() -> Self {
        Self {
            data: Vec::new(),
            marker: std::marker::PhantomData,
        }
    }
}
impl<'input> Source<'input> {
    pub(crate) fn push<T>(&mut self, arg: T)
    where
        SourceStructure<'input>: std::convert::From<T>,
    {
        self.data.push(SourceStructure::from(arg));
    }

    pub(crate) fn extend<I, T>(&mut self, iter: I)
    where
        SourceStructure<'input>: std::convert::From<T>,
        I: IntoIterator<Item = T>,
    {
        self.data
            .extend(iter.into_iter().map(|x| SourceStructure::from(x)));
    }
    pub fn iter<'a>(&'a self) -> ::std::slice::Iter<'a, SourceStructure<'input>> {
        self.data.as_slice().iter()
    }
}
impl<'input> IntoIterator for Source<'input> {
    type Item = SourceStructure<'input>;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> std::vec::IntoIter<Self::Item> {
        self.data.into_iter()
    }
}
