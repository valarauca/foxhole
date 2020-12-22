use crate::internals::{
    canonization::graph::{ChildLambda, Edge, EdgeTrait, Graph, Node, NodeIndex, NodeTrait},
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
        let span: Span = self.span.as_ref().clone();
        let mut v: Vec<ChildLambda> = vec![Box::new(move |graph,parent| {
            let id = graph.build_from_root(span);
            graph.add_edge(parent, id, StatementSpan::default());
        })];
        let lambda: ChildLambda = match self.sttm.as_ref() {
            State::Declaration(ref assign) => {
                let assign: Assign = assign.as_ref().clone();
                Box::new(move |graph, parent| {
                    let id = graph.build_from_root(assign);
                    graph.add_edge(parent, id, StatementAssign::default());
                })
            },
            State::Func(ref func) => {
                let func: FunctionDec = func.as_ref().clone();
                Box::new(move |graph, parent| {
                    let id = graph.build_from_root(func);
                    graph.add_edge(parent, id, StatementAssign::default());
                })
            },
            State::CompFunc(ref comp_func) => {
                let comp_func: CompositionalFunction = comp_func.as_ref().clone();
                Box::new(move |graph, parent| {
                    let id = graph.build_from_root(comp_func);
                    graph.add_edge(parent, id, StatementCompFuncDec::default());
                })
            },
            State::Termination(ref term) => {
                let term: Expression = term.as_ref().clone();
                Box::new(move |graph, parent| {
                    let id = graph.build_from_root(term);
                    graph.add_edge(parent, id, StatementExpr::default());
                })
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
