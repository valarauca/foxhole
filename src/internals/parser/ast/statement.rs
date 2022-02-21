use crate::internals::{
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

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Statement {
    pub sttm: Box<State>,
    pub span: Box<Span>,
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
