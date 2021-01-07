use serde::{Deserialize, Serialize};

use crate::internals::{
    canonization::graph::{
        build_typed_child_lambda, ChildLambda, Edge, EdgeTrait, Graph, Node, NodeIndex, NodeTrait,
    },
    parser::{
        ast::{
            condition::Conditional, ident::Ident, invoke::Invoke, op::Operation, template::Template,
        },
        span::{Span, Spanner},
    },
};

/// Expressions are just the meet & potatos of code.
/// random bits saying `y + 2` and what not.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Expression {
    pub kind: Box<Expr>,

    pub span: Box<Span>,
}

#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ExpressionVar;

impl EdgeTrait for ExpressionVar {
    type N = Ident;
}

#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ExpressionNum;

impl EdgeTrait for ExpressionNum {
    type N = Span;
}

#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ExpressionTemplate;

impl EdgeTrait for ExpressionTemplate {
    type N = Template;
}

#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ExpressionInvoke;

impl EdgeTrait for ExpressionInvoke {
    type N = Invoke;
}

#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ExpressionParens;

impl EdgeTrait for ExpressionParens {
    type N = Expression;
}

#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ExpressionOp;

impl EdgeTrait for ExpressionOp {
    type N = Operation;
}

#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ExpressionCond;

impl EdgeTrait for ExpressionCond {
    type N = Conditional;
}

#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ExpressionSpan;

impl EdgeTrait for ExpressionSpan {
    type N = Span;
}

impl NodeTrait for Expression {
    fn children(&self) -> Vec<ChildLambda> {
        vec![
            match self.kind.as_ref() {
                Expr::Var(ref ident) => build_typed_child_lambda::<_, ExpressionVar>(ident),
                Expr::Num(ref num) => build_typed_child_lambda::<_, ExpressionNum>(num),
                Expr::Template(ref template) => {
                    build_typed_child_lambda::<_, ExpressionTemplate>(template)
                }
                Expr::Invoke(ref invoke) => build_typed_child_lambda::<_, ExpressionInvoke>(invoke),
                Expr::Op(ref op) => build_typed_child_lambda::<_, ExpressionOp>(op),
                Expr::Parens(ref expr) => build_typed_child_lambda::<_, ExpressionParens>(expr),
                Expr::Cond(ref cond) => build_typed_child_lambda::<_, ExpressionCond>(cond),
            },
            build_typed_child_lambda::<_, ExpressionSpan>(&self.span),
        ]
    }
}

impl AsRef<Span> for Expression {
    fn as_ref(&self) -> &Span {
        &self.span
    }
}
impl Spanner for Expression {}

/// Expr stores the internal information about the expression.
/// more or less, what the expression is and what it is doing
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Expr {
    Var(Box<Ident>),

    Num(Box<Span>),

    Template(Box<Template>),

    Invoke(Box<Invoke>),

    Op(Box<Operation>),

    Parens(Box<Expression>),

    Cond(Box<Conditional>),
}
macro_rules! expr_from {
    ($TypeName: ident; { $($Variant: ident => $Interior: ident);*}) => {
        $(
        impl From<$Interior> for $TypeName {
            #[inline(always)]
            fn from(arg: $Interior) -> Self {
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

impl Expression {
    pub(in crate::internals::parser) fn new<F, S>(
        item: F,
        span: S,
    ) -> Result<Self, lrpar::Lexeme<u32>>
    where
        Expr: From<F>,
        S: FnOnce() -> Result<Span, lrpar::Lexeme<u32>>,
    {
        let span = Box::new(span()?);
        let kind = Box::new(Expr::from(item));
        Ok(Self { kind, span })
    }
}
