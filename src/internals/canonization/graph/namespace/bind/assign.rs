use crate::internals::canonization::graph::namespace::GetName;
use crate::internals::canonization::kinds::workable::{TypeData, TypeDataTrait};
use crate::internals::parser::ast::assign::Assign as AstAssign;
use crate::internals::parser::span::{Span, Spanner};

/// Bind describes a variables bind site
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Assign<'temp, 'input: 'temp> {
    internal: &'temp AstAssign<'input>,
    kind: TypeData,
}

impl<'temp, 'input: 'temp> GetName<'temp> for Assign<'temp, 'input> {
    fn get_name(&self) -> &'temp str {
        self.internal.name.get_span()
    }
}

impl<'temp, 'input: 'temp> From<&'temp AstAssign<'input>> for Assign<'temp, 'input> {
    /// build a new bind
    fn from(arg: &'temp AstAssign<'input>) -> Self {
        Self {
            kind: TypeData::from(arg.kind.as_ref()),
            internal: arg,
        }
    }
}

impl<'temp, 'input: 'temp> AsRef<TypeData> for Assign<'temp, 'input> {
    #[inline(always)]
    fn as_ref<'a>(&'a self) -> &'a TypeData {
        &self.kind
    }
}

impl<'temp, 'input: 'temp> AsMut<TypeData> for Assign<'temp, 'input> {
    #[inline(always)]
    fn as_mut<'a>(&'a mut self) -> &'a mut TypeData {
        &mut self.kind
    }
}

impl<'temp, 'input: 'temp> TypeDataTrait for Assign<'temp, 'input> {}

impl<'temp, 'input: 'temp> AsRef<Span<'input>> for Assign<'temp, 'input> {
    #[inline(always)]
    fn as_ref<'a>(&'a self) -> &'a Span<'input> {
        self.internal.as_ref()
    }
}
impl<'temp, 'input: 'temp> Spanner<'input> for Assign<'temp, 'input> {}
