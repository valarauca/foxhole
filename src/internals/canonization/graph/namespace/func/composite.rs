use crate::internals::canonization::graph::namespace::GetName;
use crate::internals::canonization::kinds::workable::{TypeData, TypeDataTrait};
use crate::internals::parser::ast::comparg::CompositionalFunction as AstCompositionalFunction;
use crate::internals::parser::span::{Span, Spanner};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CompFunction<'temp, 'input: 'temp> {
    pub internal: &'temp AstCompositionalFunction<'input>,
    kind: TypeData,
}

/*
 * Constructor
 *
 */

impl<'temp, 'input: 'temp> From<&'temp AstCompositionalFunction<'input>>
    for CompFunction<'temp, 'input>
{
    #[inline(always)]
    fn from(arg: &'temp AstCompositionalFunction<'input>) -> Self {
        Self {
            kind: TypeData::from(arg),
            internal: arg,
        }
    }
}

/*
 * Type System Boilerplate
 *
 */

impl<'temp, 'input: 'temp> AsRef<TypeData> for CompFunction<'temp, 'input> {
    #[inline(always)]
    fn as_ref<'a>(&'a self) -> &'a TypeData {
        &self.kind
    }
}

impl<'temp, 'input: 'temp> AsMut<TypeData> for CompFunction<'temp, 'input> {
    #[inline(always)]
    fn as_mut<'a>(&'a mut self) -> &'a mut TypeData {
        &mut self.kind
    }
}

impl<'temp, 'input: 'temp> TypeDataTrait for CompFunction<'temp, 'input> {}

/*
 * Way to fetch a name
 *
 */

impl<'temp, 'input: 'temp> GetName<'temp> for CompFunction<'temp, 'input> {
    fn get_name(&self) -> &'temp str {
        self.internal.name.get_span()
    }
}

/*
 * Span boilerplate
 *
 */

impl<'temp, 'input: 'temp> AsRef<Span<'input>> for CompFunction<'temp, 'input> {
    #[inline(always)]
    fn as_ref<'a>(&'a self) -> &'a Span<'input> {
        self.internal.as_ref()
    }
}

impl<'temp, 'input: 'temp> Spanner<'input> for CompFunction<'temp, 'input> {}
