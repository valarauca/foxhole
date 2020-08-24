use crate::internals::canonization::graph::namespace::GetName;
use crate::internals::canonization::kinds::workable::{TypeData, TypeDataTrait};
use crate::internals::parser::ast::args::FunctionArg as AstFunctionArg;
use crate::internals::parser::ast::ident::Ident as AstIdent;
use crate::internals::parser::span::{Span, Spanner};

/// input argument to the function
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FunctionArg<'temp, 'input: 'temp> {
    internal: &'temp AstFunctionArg<'input>,
    kind: TypeData,
}

impl<'temp, 'input: 'temp> GetName<'temp> for FunctionArg<'temp, 'input> {
    fn get_name(&self) -> &'temp str {
        self.internal.name.get_span()
    }
}

impl<'temp, 'input: 'temp> From<&'temp AstFunctionArg<'input>> for FunctionArg<'temp, 'input> {
    fn from(arg: &'temp AstFunctionArg<'input>) -> Self {
        Self {
            kind: TypeData::from(arg.kind.as_ref()),
            internal: arg,
        }
    }
}

impl<'temp, 'input: 'temp> AsRef<TypeData> for FunctionArg<'temp, 'input> {
    #[inline(always)]
    fn as_ref<'a>(&'a self) -> &'a TypeData {
        &self.kind
    }
}

impl<'temp, 'input: 'temp> AsMut<TypeData> for FunctionArg<'temp, 'input> {
    #[inline(always)]
    fn as_mut<'a>(&'a mut self) -> &'a mut TypeData {
        &mut self.kind
    }
}

impl<'temp, 'input: 'temp> TypeDataTrait for FunctionArg<'temp, 'input> {}

impl<'temp, 'input: 'temp> AsRef<Span<'input>> for FunctionArg<'temp, 'input> {
    #[inline(always)]
    fn as_ref<'a>(&'a self) -> &'a Span<'input> {
        self.internal.as_ref()
    }
}
impl<'temp, 'input: 'temp> Spanner<'input> for FunctionArg<'temp, 'input> {}
