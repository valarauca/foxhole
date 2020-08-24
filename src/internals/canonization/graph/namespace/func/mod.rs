use crate::internals::canonization::graph::namespace::GetName;
use crate::internals::canonization::kinds::workable::{TypeData, TypeDataTrait};
use crate::internals::parser::ast::comparg::CompositionalFunction as AstCompositionalFunction;
use crate::internals::parser::ast::func::FunctionDec as AstFunctionDec;
use crate::internals::parser::span::{Span, Spanner};

mod composite;
pub use self::composite::CompFunction;
mod function;
pub use self::function::Function;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Func<'temp, 'input: 'temp> {
    Func(Function<'temp, 'input>),
    Comp(CompFunction<'temp, 'input>),
}

/*
 * Constructor boilerplate
 *
 */
impl<'temp, 'input: 'temp> From<Function<'temp, 'input>> for Func<'temp, 'input> {
    fn from(arg: Function<'temp, 'input>) -> Self {
        Self::Func(arg)
    }
}

impl<'temp, 'input: 'temp> From<CompFunction<'temp, 'input>> for Func<'temp, 'input> {
    #[inline(always)]
    fn from(arg: CompFunction<'temp, 'input>) -> Self {
        Self::Comp(arg)
    }
}

impl<'temp, 'input: 'temp> From<&'temp AstFunctionDec<'input>> for Func<'temp, 'input> {
    fn from(arg: &'temp AstFunctionDec<'input>) -> Self {
        Self::from(Function::from(arg))
    }
}

impl<'temp, 'input: 'temp> From<&'temp AstCompositionalFunction<'input>> for Func<'temp, 'input> {
    fn from(arg: &'temp AstCompositionalFunction<'input>) -> Self {
        Self::from(CompFunction::from(arg))
    }
}

/*
 * Type System Boilerplate
 *
 */

impl<'temp, 'input: 'temp> AsRef<TypeData> for Func<'temp, 'input> {
    fn as_ref<'a>(&'a self) -> &'a TypeData {
        match self {
            &Self::Func(ref func) => func.as_ref(),
            &Self::Comp(ref comp) => comp.as_ref(),
        }
    }
}

impl<'temp, 'input: 'temp> AsMut<TypeData> for Func<'temp, 'input> {
    fn as_mut<'a>(&'a mut self) -> &'a mut TypeData {
        match self {
            &mut Self::Func(ref mut func) => func.as_mut(),
            &mut Self::Comp(ref mut comp) => comp.as_mut(),
        }
    }
}

impl<'temp, 'input: 'temp> TypeDataTrait for Func<'temp, 'input> {}

/*
 * Way to fetch a name
 *
 */

impl<'temp, 'input: 'temp> GetName<'temp> for Func<'temp, 'input> {
    fn get_name(&self) -> &'temp str {
        match self {
            &Self::Func(ref func) => func.get_name(),
            &Self::Comp(ref comp) => comp.get_name(),
        }
    }
}

/*
 * Span boilerplate
 *
 */

impl<'temp, 'input: 'temp> AsRef<Span<'input>> for Func<'temp, 'input> {
    #[inline(always)]
    fn as_ref<'a>(&'a self) -> &'a Span<'input> {
        match self {
            &Self::Func(ref func) => func.as_ref(),
            &Self::Comp(ref comp) => comp.as_ref(),
        }
    }
}

impl<'temp, 'input: 'temp> Spanner<'input> for Func<'temp, 'input> {}
