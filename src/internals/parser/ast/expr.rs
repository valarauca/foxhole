use serde::{Deserialize, Serialize};

use crate::internals::{
    canonization::graph::{ChildLambda, Edge, EdgeTrait, Graph, Node, NodeIndex, NodeTrait},
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
        let lambda: ChildLambda = match self.kind.as_ref() {
            Expr::Var(ref ident) => {
                let ident: Ident = ident.as_ref().clone();
                Box::new(move |graph, parent| {
                    let id = graph.build_from_root(ident);
                    graph.add_edge(parent, id, ExpressionVar::default());
                })
            }
            Expr::Num(ref num) => {
                let num: Span = num.as_ref().clone();
                Box::new(move |graph, parent| {
                    let id = graph.build_from_root(num);
                    graph.add_edge(parent, id, ExpressionNum::default());
                })
            }
            Expr::Template(ref template) => {
                let template: Template = template.as_ref().clone();
                Box::new(move |graph, parent| {
                    let id = graph.build_from_root(template);
                    graph.add_edge(parent, id, ExpressionTemplate::default());
                })
            }
            Expr::Invoke(ref invoke) => {
                let invoke: Invoke = invoke.as_ref().clone();
                Box::new(move |graph, parent| {
                    let id = graph.build_from_root(invoke);
                    graph.add_edge(parent, id, ExpressionInvoke::default());
                })
            }
            Expr::Op(ref op) => {
                let op: Operation = op.as_ref().clone();
                Box::new(move |graph, parent| {
                    let id = graph.build_from_root(op);
                    graph.add_edge(parent, id, ExpressionOp::default());
                })
            }
            Expr::Parens(ref expr) => {
                let expr: Expression = expr.as_ref().clone();
                Box::new(move |graph, parent| {
                    let id = graph.build_from_root(expr);
                    graph.add_edge(parent, id, ExpressionParens::default());
                })
            }
            Expr::Cond(ref cond) => {
                let cond: Conditional = cond.as_ref().clone();
                Box::new(move |graph, parent| {
                    let id = graph.build_from_root(cond);
                    graph.add_edge(parent, id, ExpressionCond::default());
                })
            }
        };
        let span: Span = self.span.as_ref().clone();
        vec![
            lambda,
            Box::new(move |graph, parent| {
                let id = graph.build_from_root(span);
                graph.add_edge(parent, id, ExpressionSpan::default());
            }),
        ]
    }
}

impl AsRef<Span> for Expression {
    fn as_ref(&self) -> &Span {
        &self.span
    }
}
impl Spanner for Expression {
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
