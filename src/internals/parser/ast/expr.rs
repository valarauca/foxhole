
use crate::internals::parser::span::{Span, Spanner};
use crate::internals::parser::ast::ident::{Identifier};
use crate::internals::parser::ast::template::{Template};

/// Expressions are just the meet & potatos of code.
/// random bits saying `x := y +2;` and what not.
#[derive(Clone, Debug)]
pub struct Expression<'input> {
    pub kind: Expr<'input>,
    pub span: Span<'input>,
}
impl<'input> AsRef<Span<'input>> for Expression<'input> {
    fn as_ref(&self) -> &Span<'input> {
        &self.span
    }
}
impl<'input> Spanner<'input> for Expression<'input> {}

/// Expr stores the internal information about the expression.
/// more or less, what the expression is and what it is doing
#[derive(Clone, Debug)]
pub enum Expr<'input> {
    Ident(Box<Identifier<'input>>),
    Number(Box<Span<'input>>),
    TemplateVar(Box<Template<'input>>),
    Parens(Box<Expression<'input>>),
    Binomial(Box<BinomialExpression<'input>>),
    Call(Box<FunctionInvocation<'input>>),
    Cond(Box<Conditional<'input>>),
    Assign(Box<Assignment<'input>>),
    UnaryOp(Box<UnaryExpression<'input>>),
}

impl<'input> Expression<'input> {
    /// create a new ident
    pub fn ident(ident: Identifier<'input>) -> Expression<'input> {
        let span = ident.as_ref().clone();
        Expression {
            kind: Expr::Ident(Box::new(ident)),
            span,
        }
    }

    /// creates a new number
    pub fn number(span: Span<'input>) -> Expression<'input> {
        Expression {
            span: span.clone(),
            kind: Expr::Number(Box::new(span)),
        }
    }

    /// creates a new template
    pub fn template(template: Template<'input>) -> Expression<'input> {
        Expression {
            span: template.as_ref().clone(),
            kind: Expr::TemplateVar(Box::new(template)),
        }
    }

    /// expression wrapped in parentheses
    pub fn parens(expr: Expression<'input>, span: Span<'input>) -> Expression<'input> {
        Expression {
            span: span,
            kind: Expr::Parens(Box::new(expr)),
        }
    }

    pub fn call(
        name: Identifier<'input>,
        args: Vec<Expression<'input>>,
        span: Span<'input>,
    ) -> Expression<'input> {
        Expression {
            span: span.clone(),
            kind: Expr::Call(Box::new(FunctionInvocation { span, name, args })),
        }
    }
    pub fn cond(
        cond: Expression<'input>,
        t: Expression<'input>,
        f: Expression<'input>,
        span: Span<'input>,
    ) -> Expression<'input> {
        Expression {
            span: span.clone(),
            kind: Expr::Cond(Box::new(Conditional { span, cond, t, f })),
        }
    }

    pub fn assign(
        ident: Identifier<'input>,
        expr: Expression<'input>,
        span: Span<'input>,
    ) -> Expression<'input> {
        Expression {
            span: span.clone(),
            kind: Expr::Assign(Box::new(Assignment {
                var: ident,
                expr,
                span,
            })),
        }
    }

    /*
     * Make internal types less tedious to define
     *
     */
    fn binomial(
        left: Expression<'input>,
        right: Expression<'input>,
        span: Span<'input>,
        op: BinomialOperator,
    ) -> Expression<'input> {
        Expression {
            span: span.clone(),
            kind: Expr::Binomial(Box::new(BinomialExpression {
                span,
                left,
                op,
                right,
            })),
        }
    }

    pub fn add(
        left: Expression<'input>,
        right: Expression<'input>,
        span: Span<'input>,
    ) -> Expression<'input> {
        Self::binomial(left, right, span, BinomialOperator::Addition)
    }
    pub fn sub(
        left: Expression<'input>,
        right: Expression<'input>,
        span: Span<'input>,
    ) -> Expression<'input> {
        Self::binomial(left, right, span, BinomialOperator::Subtraction)
    }
    pub fn mul(
        left: Expression<'input>,
        right: Expression<'input>,
        span: Span<'input>,
    ) -> Expression<'input> {
        Self::binomial(left, right, span, BinomialOperator::Multiplication)
    }
    pub fn div(
        left: Expression<'input>,
        right: Expression<'input>,
        span: Span<'input>,
    ) -> Expression<'input> {
        Self::binomial(left, right, span, BinomialOperator::Division)
    }
    pub fn eq(
        left: Expression<'input>,
        right: Expression<'input>,
        span: Span<'input>,
    ) -> Expression<'input> {
        Self::binomial(left, right, span, BinomialOperator::Equal)
    }
    pub fn not_eq(
        left: Expression<'input>,
        right: Expression<'input>,
        span: Span<'input>,
    ) -> Expression<'input> {
        Self::binomial(left, right, span, BinomialOperator::NotEqual)
    }
    pub fn lt(
        left: Expression<'input>,
        right: Expression<'input>,
        span: Span<'input>,
    ) -> Expression<'input> {
        Self::binomial(left, right, span, BinomialOperator::LessThan)
    }
    pub fn gt(
        left: Expression<'input>,
        right: Expression<'input>,
        span: Span<'input>,
    ) -> Expression<'input> {
        Self::binomial(left, right, span, BinomialOperator::GreaterThan)
    }
    pub fn lteq(
        left: Expression<'input>,
        right: Expression<'input>,
        span: Span<'input>,
    ) -> Expression<'input> {
        Self::binomial(left, right, span, BinomialOperator::LessThanEqualToo)
    }
    pub fn gteq(
        left: Expression<'input>,
        right: Expression<'input>,
        span: Span<'input>,
    ) -> Expression<'input> {
        Self::binomial(left, right, span, BinomialOperator::GreaterThanEqualToo)
    }
    pub fn and(
        left: Expression<'input>,
        right: Expression<'input>,
        span: Span<'input>,
    ) -> Expression<'input> {
        Self::binomial(left, right, span, BinomialOperator::And)
    }
    pub fn or(
        left: Expression<'input>,
        right: Expression<'input>,
        span: Span<'input>,
    ) -> Expression<'input> {
        Self::binomial(left, right, span, BinomialOperator::Or)
    }
    pub fn xor(
        left: Expression<'input>,
        right: Expression<'input>,
        span: Span<'input>,
    ) -> Expression<'input> {
        Self::binomial(left, right, span, BinomialOperator::Xor)
    }
    pub fn hash(
        left: Expression<'input>,
        right: Expression<'input>,
        span: Span<'input>,
    ) -> Expression<'input> {
        Self::binomial(left, right, span, BinomialOperator::ForEach)
    }

    /*
     * Unary Operations
     *
     */

    fn unary_op(
        expr: Expression<'input>,
        span: Span<'input>,
        op: UnaryOperator,
    ) -> Expression<'input> {
        Expression {
            span: span.clone(),
            kind: Expr::UnaryOp(Box::new(UnaryExpression { span, expr, op })),
        }
    }

    pub fn max(expr: Expression<'input>, span: Span<'input>) -> Expression<'input> {
        Self::unary_op(expr, span, UnaryOperator::Max)
    }
    pub fn min(expr: Expression<'input>, span: Span<'input>) -> Expression<'input> {
        Self::unary_op(expr, span, UnaryOperator::Min)
    }
    fn not(expr: Expression<'input>, span: Span<'input>) -> Expression<'input> {
        Self::unary_op(expr, span, UnaryOperator::Not)
    }
    pub fn sum(expr: Expression<'input>, span: Span<'input>) -> Expression<'input> {
        Self::unary_op(expr, span, UnaryOperator::Sum)
    }
    pub fn count(expr: Expression<'input>, span: Span<'input>) -> Expression<'input> {
        Self::unary_op(expr, span, UnaryOperator::Count)
    }
    pub fn dice6(expr: Expression<'input>, span: Span<'input>) -> Expression<'input> {
        Self::unary_op(expr, span, UnaryOperator::Dice6)
    }
    pub fn dice3(expr: Expression<'input>, span: Span<'input>) -> Expression<'input> {
        Self::unary_op(expr, span, UnaryOperator::Dice3)
    }
    pub fn dice10(expr: Expression<'input>, span: Span<'input>) -> Expression<'input> {
        Self::unary_op(expr, span, UnaryOperator::Dice10)
    }
    pub fn dice12(expr: Expression<'input>, span: Span<'input>) -> Expression<'input> {
        Self::unary_op(expr, span, UnaryOperator::Dice12)
    }
}

/*
 * Defining expresions with 2 arguments and an operator
 *
 */

/// BinomialExpressions are expressions which require 2 expressions.
#[derive(Clone, Debug)]
pub struct BinomialExpression<'input> {
    pub span: Span<'input>,
    pub left: Expression<'input>,
    pub op: BinomialOperator,
    pub right: Expression<'input>,
}
impl<'input> AsRef<Span<'input>> for BinomialExpression<'input> {
    fn as_ref(&self) -> &Span<'input> {
        &self.span
    }
}
impl<'input> Spanner<'input> for BinomialExpression<'input> {}

/// The various operators for Bionomial Expressions
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum BinomialOperator {
    Addition,
    Subtraction,
    Division,
    Multiplication,
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessThanEqualToo,
    GreaterThanEqualToo,
    And,
    Or,
    Xor,
    ForEach,
}

/*
 * Defining Expressions with require invoking a function
 *
 */

/// Want to call a function, this is the argument that is
/// used.
#[derive(Clone, Debug)]
pub struct FunctionInvocation<'input> {
    pub span: Span<'input>,
    pub name: Identifier<'input>,
    pub args: Vec<Expression<'input>>,
}
impl<'input> AsRef<Span<'input>> for FunctionInvocation<'input> {
    fn as_ref(&self) -> &Span<'input> {
        &self.span
    }
}
impl<'input> Spanner<'input> for FunctionInvocation<'input> {}

/*
 *
 * Defining Conditional Expressions
 *
 */
/// Basically, and if statement. All statements require
/// both directions are declared.
#[derive(Clone, Debug)]
pub struct Conditional<'input> {
    pub cond: Expression<'input>,
    pub t: Expression<'input>,
    pub f: Expression<'input>,
    pub span: Span<'input>,
}
impl<'input> AsRef<Span<'input>> for Conditional<'input> {
    fn as_ref(&self) -> &Span<'input> {
        &self.span
    }
}
impl<'input> Spanner<'input> for Conditional<'input> {}

/*
 *
 * Unary Expressions
 *
 */
/// Operations that invole a single sigal
#[derive(Clone, Debug)]
pub struct UnaryExpression<'input> {
    pub span: Span<'input>,
    pub expr: Expression<'input>,
    pub op: UnaryOperator,
}
impl<'input> AsRef<Span<'input>> for UnaryExpression<'input> {
    fn as_ref(&self) -> &Span<'input> {
        &self.span
    }
}
impl<'input> Spanner<'input> for UnaryExpression<'input> {}

#[derive(Clone, Debug)]
pub enum UnaryOperator {
    Not,
    Count,
    Max,
    Min,
    Sum,
    Dice3,
    Dice6,
    Dice10,
    Dice12,
}

/*
 * Assigning variables
 *
 */
#[derive(Clone, Debug)]
pub struct Assignment<'input> {
    pub span: Span<'input>,
    pub var: Identifier<'input>,
    pub expr: Expression<'input>,
}
impl<'input> AsRef<Span<'input>> for Assignment<'input> {
    fn as_ref(&self) -> &Span<'input> {
        &self.span
    }
}
impl<'input> Spanner<'input> for Assignment<'input> {}
