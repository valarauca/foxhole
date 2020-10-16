use serde::{Deserialize, Serialize};

use crate::internals::parser::ast::expr::Expression;
use crate::internals::parser::ast::ident::Ident;
use crate::internals::parser::span::{Span, Spanner};

/// Invoking a function
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Invoke {
    pub name: Box<Ident>,

    pub args: Box<[Expression]>,

    pub span: Box<Span>,
}
impl AsRef<Span> for Invoke {
    fn as_ref(&self) -> &Span {
        &self.span
    }
}
impl Spanner for Invoke {
    fn fields(&self) {
        self.set_id();
        self.name.fields();
        for arg in self.args.as_ref().iter() {
            arg.fields();
        }
    }
}

impl Invoke {
    pub(in crate::internals::parser) fn new<I, F>(
        name: Ident,
        args: I,
        span: F,
    ) -> Result<Self, lrpar::Lexeme<u32>>
    where
        I: IntoIterator<Item = Expression>,
        F: FnOnce() -> Result<Span, lrpar::Lexeme<u32>>,
    {
        let span = Box::new(span()?);
        let args = args
            .into_iter()
            .collect::<Vec<Expression>>()
            .into_boxed_slice();
        let name = Box::new(name);
        Ok(Self { name, args, span })
    }
}
