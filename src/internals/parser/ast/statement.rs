use crate::internals::{
    canonization::graph::{
        build_data_child_lambda, build_typed_child_lambda, ChildLambda, Edge, EdgeTrait, Graph,
        Node, NodeIndex, NodeTrait,
    },
    parser::{
        ast::{
            assign::Assign, comparg::CompositionalFunction, expr::Expression, func::FunctionDec,
        },
        span::{Span, Spanner},
    },
};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Body {
    pub body: Vec<Statement>,
    pub span: Box<Span>,
}

impl AsRef<Span> for Body {
    fn as_ref(&self) -> &Span {
        &self.span
    }
}

impl Spanner for Body {}

impl Body {
    pub(in crate::internals::parser) fn new<S>(
        body: Vec<Statement>,
        span: S,
    ) -> Result<Self, lrpar::Lexeme<u32>>
    where
        S: FnOnce() -> Result<Span, lrpar::Lexeme<u32>>,
    {
        let span = Box::new(span()?);
        Ok(Self { body, span })
    }
}

#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct StatementNum(usize);

impl EdgeTrait for StatementNum {
    type N = Statement;
}

impl NodeTrait for Body {
    fn children(&self) -> Vec<ChildLambda> {
        self.body
            .iter()
            .enumerate()
            .map(|(pos, statement)| build_data_child_lambda(statement, StatementNum(pos)))
            .collect()
    }
}

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
        let mut v = vec![build_typed_child_lambda::<_, StatementSpan>(&self.span)];
        let lambda = match self.sttm.as_ref() {
            State::Declaration(ref assign) => {
                build_typed_child_lambda::<_, StatementAssign>(assign)
            }
            State::Func(ref func) => build_typed_child_lambda::<_, StatementFuncDec>(func),
            State::CompFunc(ref comp_func) => {
                build_typed_child_lambda::<_, StatementCompFuncDec>(comp_func)
            }
            State::Termination(ref term) => build_typed_child_lambda::<_, StatementExpr>(term),
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

impl Spanner for Statement {}

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
