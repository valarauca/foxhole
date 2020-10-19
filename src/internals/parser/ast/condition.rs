use serde::{Deserialize, Serialize};

use crate::internals::parser::ast::expr::Expression;
use crate::internals::parser::span::{Span, Spanner};

/// Conditionals manage things like `if else`
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Conditional {
    pub condition: Box<Expression>,

    pub true_case: Box<Expression>,

    pub false_case: Box<Expression>,

    pub span: Box<Span>,
}
impl AsRef<Span> for Conditional {
    fn as_ref(&self) -> &Span {
        &self.span
    }
}
impl Spanner for Conditional {
    fn fields(&self) {
        self.set_id();
        self.condition.fields();
        self.true_case.fields();
        self.false_case.fields();
    }
}

impl Conditional {
    pub(in crate::internals::parser) fn new<S>(
        condition: Expression,
        true_case: Expression,
        false_case: Expression,
        span: S,
    ) -> Result<Self, lrpar::Lexeme<u32>>
    where
        S: FnOnce() -> Result<Span, lrpar::Lexeme<u32>>,
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
