use serde::{Deserialize, Serialize};

use super::{GetInternalExpression, InternalExpression};

use crate::internals::parser::ast::expr::Expression;
use crate::internals::parser::ast::ident::Ident;
use crate::internals::parser::ast::kind::Kind;
use crate::internals::parser::span::{Span, Spanner};

/// Assign represents an assignment
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Assign {
    
    pub name: Box<Ident>,
    
    pub expr: Box<Expression>,
    pub kind: Box<Option<Kind>>,
    
    pub span: Box<Span>,
}
impl GetInternalExpression for Assign {
    /// returns the defining expression
    fn get_expr<'a>(&'a self) -> Option<InternalExpression<'a>> {
        Some(InternalExpression::Single(self.expr.as_ref()))
    }
}
impl AsRef<Span> for Assign {
    fn as_ref(&self) -> &Span {
        &self.span
    }
}
impl Spanner for Assign {
    fn fields(&self) {
        self.set_id();
        self.name.fields();
        self.expr.fields();
    }
}

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
