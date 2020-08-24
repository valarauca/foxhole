use serde::{Deserialize, Serialize};

use super::{GetInternalExpression, InternalExpression};
use crate::internals::parser::ast::condition::Conditional;
use crate::internals::parser::ast::ident::Ident;
use crate::internals::parser::ast::invoke::Invoke;
use crate::internals::parser::ast::op::Operation;
use crate::internals::parser::ast::template::Template;
use crate::internals::parser::span::{Span, Spanner};

/// Expressions are just the meet & potatos of code.
/// random bits saying `y + 2` and what not.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Expression<'input> {
    #[serde(borrow)]
    pub kind: Box<Expr<'input>>,
    #[serde(borrow)]
    pub span: Box<Span<'input>>,
}
impl<'input> AsRef<Span<'input>> for Expression<'input> {
    fn as_ref(&self) -> &Span<'input> {
        &self.span
    }
}
impl<'input> Spanner<'input> for Expression<'input> {
    fn fields(&self) {
        self.set_id();
        match self.kind.as_ref() {
            &Expr::Var(ref a) => a.fields(),
            &Expr::Num(ref b) => b.fields(),
            &Expr::Template(ref c) => c.fields(),
            &Expr::Invoke(ref d) => d.fields(),
            &Expr::Op(ref e) => e.fields(),
            &Expr::Parens(ref f) => f.fields(),
            &Expr::Cond(ref g) => g.fields(),
        }
    }
}

impl<'input> GetInternalExpression<'input> for Expression<'input> {
    fn get_expr<'a>(&'a self) -> Option<InternalExpression<'a, 'input>> {
        match self.kind.as_ref() {
            &Expr::Cond(ref cond) => cond.get_expr(),
            &Expr::Op(ref op) => op.get_expr(),
            &Expr::Parens(ref par) => par.get_expr(),
            _ => None,
        }
    }
}

/// Expr stores the internal information about the expression.
/// more or less, what the expression is and what it is doing
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Expr<'input> {
    #[serde(borrow)]
    Var(Box<Ident<'input>>),
    #[serde(borrow)]
    Num(Box<Span<'input>>),
    #[serde(borrow)]
    Template(Box<Template<'input>>),
    #[serde(borrow)]
    Invoke(Box<Invoke<'input>>),
    #[serde(borrow)]
    Op(Box<Operation<'input>>),
    #[serde(borrow)]
    Parens(Box<Expression<'input>>),
    #[serde(borrow)]
    Cond(Box<Conditional<'input>>),
}
macro_rules! expr_from {
    ($TypeName: ident; { $($Variant: ident => $Interior: ident);*}) => {
        $(
        impl<'input> From<$Interior<'input>> for $TypeName<'input> {
            #[inline(always)]
            fn from(arg: $Interior<'input>) -> Self {
                Self::$Variant(Box::new(arg))
            }
        }
        )*
    };
}
expr_from! { Expr; {
    Cond => Conditional;
    Var => Ident;
    Num => Span;
    Template => Template;
    Invoke => Invoke;
    Op => Operation;
    Parens => Expression
}}

impl<'input> Expression<'input> {
    pub(in crate::internals::parser) fn new<F, S>(
        item: F,
        span: S,
    ) -> Result<Self, lrpar::Lexeme<u32>>
    where
        Expr<'input>: From<F>,
        S: FnOnce() -> Result<Span<'input>, lrpar::Lexeme<u32>>,
    {
        let span = Box::new(span()?);
        let kind = Box::new(Expr::from(item));
        Ok(Self { kind, span })
    }
}
