use crate::internals::{
    canonization::graph::{ChildLambda, Edge, EdgeTrait, Graph, Node, NodeIndex, NodeTrait, build_typed_child_lambda},
    parser::{
        span::{Span,Spanner},
        ast::{func::FunctionDec, expr::Expression, comparg::CompositionalFunction, assign::Assign},
    },
};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Statement {
    pub sttm: Box<State>,
    pub span: Box<Span>,
}

#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct StatementSpan;

impl EdgeTrait for StatementSpan {
    type N = Span;
}

#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct StatementAssign;

impl EdgeTrait for StatementAssign {
    type N = Assign;
}

#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct StatementFuncDec;

impl EdgeTrait for StatementFuncDec {
    type N = FunctionDec;
}

#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct StatementCompFuncDec;

impl EdgeTrait for StatementCompFuncDec {
    type N = CompositionalFunction;
}

#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct StatementExpr;

impl EdgeTrait for StatementExpr {
    type N = Expression;
}


impl NodeTrait for Statement {
    fn children(&self) -> Vec<ChildLambda> {
        let mut v = vec![build_typed_child_lambda::<_,StatementSpan>(&self.span)];
        let lambda = match self.sttm.as_ref() {
            State::Declaration(ref assign) => {
                build_typed_child_lambda::<_,StatementAssign>(assign)
            },
            State::Func(ref func) => {
                build_typed_child_lambda::<_,StatementFuncDec>(func)
            },
            State::CompFunc(ref comp_func) => {
                build_typed_child_lambda::<_,StatementCompFuncDec>(comp_func)
            },
            State::Termination(ref term) => {
                build_typed_child_lambda::<_,StatementExpr>(term)
            },
        };
        v.push(lambda);
        v
    }
}


impl AsRef<Span> for Statement {
    fn as_ref(&self) -> &Span {
        &self.span
    }
}

impl Spanner for Statement {
    fn fields(&self) {
        self.set_id();
        match self.sttm.as_ref() {
            State::Declaration(ref assign) => assign.fields(),
            State::Func(ref func) => func.fields(),
            State::CompFunc(ref comp) => comp.fields(),
            State::Termination(ref expr) => expr.fields(),
        }
    }
}

impl Statement {
    pub(in crate::internals::parser) fn new<I, S>(
        item: I,
        span: S,
    ) -> Result<Self, lrpar::Lexeme<u32>>
    where
        S: FnOnce() -> Result<Span, lrpar::Lexeme<u32>>,
        State: From<I>,
    {
        let span = Box::new(span()?);
        let sttm = Box::new(State::from(item));
        Ok(Self { sttm, span })
    }
}

stuff! {
    Name: State;
    Trait: StateTrait;
    From: {
        Assign => Declaration => is_dec => get_dec,
        FunctionDec => Func => is_func => get_func,
        CompositionalFunction => CompFunc => is_comp_func => get_comp_func,
        Expression => Termination => is_term => get_term,
    }
}

impl AsRef<State> for Statement {
    #[inline(always)]
    fn as_ref<'a>(&'a self) -> &'a State {
        self.sttm.as_ref()
    }
}

impl StateTrait for Statement {}
