use serde::{Deserialize, Serialize};

use crate::internals::{
    parser::{
        span::{Span,Spanner},
        ast::{
            condition::Conditional,
            ident::Ident,
            invoke::Invoke,
            op::Operation,
            template::Template,
        }
    },
    canonization::graph::{
        EdgeTrait,NodeTrait,Graph,Node,Edge,NodeIndex,ChildLambda,
    }
};

/// Expressions are just the meet & potatos of code.
/// random bits saying `y + 2` and what not.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Expression {
    pub kind: Box<Expr>,

    pub span: Box<Span>,
}
impl NodeTrait for Expression { }

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
