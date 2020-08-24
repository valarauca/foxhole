use crate::internals::canonization::graph::namespace::GetName;
use crate::internals::canonization::kinds::workable::{TypeData, TypeDataTrait};
use crate::internals::parser::ast::args::FunctionArg as AstFunctionArg;
use crate::internals::parser::ast::assign::Assign as AstAssign;
use crate::internals::parser::span::{Span, Spanner};

mod args;
pub use self::args::FunctionArg;
mod assign;
pub use self::assign::Assign;

/// Variable is a top level "here is a value within a function" argument
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Variable<'temp, 'input: 'temp> {
    InputArg(FunctionArg<'temp, 'input>),
    Declaration(Assign<'temp, 'input>),
}

/*
 * Permit returning variable names
 *
 */

/*
 * Permit getting the name of the variable
 *
 */

impl<'temp, 'input: 'temp> GetName<'temp> for Variable<'temp, 'input> {
    fn get_name(&self) -> &'temp str {
        match self {
            &Variable::InputArg(ref arg) => arg.get_name(),
            &Variable::Declaration(ref arg) => arg.get_name(),
        }
    }
}

/*
 * Allow construction from internal sub-types
 *
 */

impl<'temp, 'input: 'temp> From<FunctionArg<'temp, 'input>> for Variable<'temp, 'input> {
    #[inline]
    fn from(arg: FunctionArg<'temp, 'input>) -> Self {
        Self::InputArg(arg)
    }
}
impl<'temp, 'input: 'temp> From<Assign<'temp, 'input>> for Variable<'temp, 'input> {
    #[inline]
    fn from(arg: Assign<'temp, 'input>) -> Self {
        Self::Declaration(arg)
    }
}
impl<'temp, 'input: 'temp> From<&'temp AstFunctionArg<'input>> for Variable<'temp, 'input> {
    #[inline]
    fn from(arg: &'temp AstFunctionArg<'input>) -> Self {
        Self::from(FunctionArg::from(arg))
    }
}
impl<'temp, 'input: 'temp> From<&'temp AstAssign<'input>> for Variable<'temp, 'input> {
    #[inline]
    fn from(arg: &'temp AstAssign<'input>) -> Self {
        Self::from(Assign::from(arg))
    }
}

/*
 * Ensure This implements TypeData
 *
 */

impl<'temp, 'input: 'temp> AsRef<TypeData> for Variable<'temp, 'input> {
    fn as_ref<'a>(&'a self) -> &'a TypeData {
        match self {
            &Variable::InputArg(ref arg) => arg.as_ref(),
            &Variable::Declaration(ref arg) => arg.as_ref(),
        }
    }
}

impl<'temp, 'input: 'temp> AsMut<TypeData> for Variable<'temp, 'input> {
    fn as_mut<'a>(&'a mut self) -> &'a mut TypeData {
        match self {
            &mut Variable::InputArg(ref mut arg) => arg.as_mut(),
            &mut Variable::Declaration(ref mut arg) => arg.as_mut(),
        }
    }
}

impl<'temp, 'input: 'temp> TypeDataTrait for Variable<'temp, 'input> {}

/*
 * Ensure this implements span
 *
 */

impl<'temp, 'input: 'temp> AsRef<Span<'input>> for Variable<'temp, 'input> {
    fn as_ref<'a>(&'a self) -> &'a Span<'input> {
        match self {
            &Variable::InputArg(ref arg) => arg.as_ref(),
            &Variable::Declaration(ref arg) => arg.as_ref(),
        }
    }
}
impl<'temp, 'input: 'temp> Spanner<'input> for Variable<'temp, 'input> {}
