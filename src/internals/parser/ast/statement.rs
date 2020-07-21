use serde::{Deserialize, Serialize};

use crate::internals::parser::ast::assign::Assign;
use crate::internals::parser::ast::comparg::CompositionalFunction;
use crate::internals::parser::ast::expr::Expression;
use crate::internals::parser::ast::func::FunctionDec;
use crate::internals::parser::span::{Span, Spanner};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Statement<'input> {
    #[serde(borrow)]
    pub sttm: Box<State<'input>>,
    #[serde(borrow)]
    pub span: Box<Span<'input>>,
}

impl<'input> AsRef<Span<'input>> for Statement<'input> {
    fn as_ref(&self) -> &Span<'input> {
        &self.span
    }
}

impl<'input> Spanner<'input> for Statement<'input> {
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

impl<'input> Statement<'input> {
    pub(in crate::internals::parser) fn new<I, S>(
        item: I,
        span: S,
    ) -> Result<Self, lrpar::Lexeme<u32>>
    where
        S: FnOnce() -> Result<Span<'input>, lrpar::Lexeme<u32>>,
        State<'input>: From<I>,
    {
        let span = Box::new(span()?);
        let sttm = Box::new(State::from(item));
        Ok(Self { sttm, span })
    }
}

stuff! {
    Name: State;
    Trait: StateTrait;
    Lifetime: 'input;
    From: {
        Assign<'input> => Declaration => is_dec => get_dec,
        FunctionDec<'input> => Func => is_func => get_func,
        CompositionalFunction<'input> => CompFunc => is_comp_func => get_comp_func,
        Expression<'input> => Termination => is_term => get_term,
    }
}

impl<'input> AsRef<State<'input>> for Statement<'input> {
    #[inline(always)]
    fn as_ref<'a>(&'a self) -> &'a State<'input> {
        self.sttm.as_ref()
    }
}

impl<'input> StateTrait<'input> for Statement<'input> {}
