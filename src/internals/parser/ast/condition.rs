use serde::{Deserialize, Serialize};

use super::{GetInternalExpression, InternalExpression};

use crate::internals::parser::ast::expr::Expression;
use crate::internals::parser::span::{Span, Spanner};

/// Conditionals manage things like `if else`
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Conditional<'input> {
    #[serde(borrow)]
    pub condition: Box<Expression<'input>>,
    #[serde(borrow)]
    pub true_case: Box<Expression<'input>>,
    #[serde(borrow)]
    pub false_case: Box<Expression<'input>>,
    #[serde(borrow)]
    pub span: Box<Span<'input>>,
}
impl<'input> GetInternalExpression<'input> for Conditional<'input> {
    fn get_expr<'a>(&'a self) -> Option<InternalExpression<'a, 'input>> {
        Some(InternalExpression::Conditional {
            cond: self.condition.as_ref(),
            true_case: self.true_case.as_ref(),
            false_case: self.false_case.as_ref(),
        })
    }
}
impl<'input> AsRef<Span<'input>> for Conditional<'input> {
    fn as_ref(&self) -> &Span<'input> {
        &self.span
    }
}
impl<'input> Spanner<'input> for Conditional<'input> {
    fn fields(&self) {
        self.set_id();
        self.condition.fields();
        self.true_case.fields();
        self.false_case.fields();
    }
}

impl<'input> Conditional<'input> {
    pub(in crate::internals::parser) fn new<S>(
        condition: Expression<'input>,
        true_case: Expression<'input>,
        false_case: Expression<'input>,
        span: S,
    ) -> Result<Self, lrpar::Lexeme<u32>>
    where
        S: FnOnce() -> Result<Span<'input>, lrpar::Lexeme<u32>>,
    {
        let span = Box::new(span()?);
        let condition = Box::new(condition);
        let true_case = Box::new(true_case);
        let false_case = Box::new(false_case);
        Ok(Self {
            condition,
            true_case,
            false_case,
            span,
        })
    }
}
