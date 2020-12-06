use serde::{Deserialize, Serialize};

use crate::internals::parser::ast::assign::Assign;
use crate::internals::parser::ast::comparg::CompositionalFunction;
use crate::internals::parser::ast::expr::Expression;
use crate::internals::parser::ast::func::FunctionDec;
use crate::internals::parser::span::{Span, Spanner};

use crate::internals::canonization::graph::NodeTrait;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Statement {
    pub sttm: Box<State>,
    pub span: Box<Span>,
}

impl NodeTrait for Statement {}

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
