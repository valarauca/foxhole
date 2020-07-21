use crate::internals::canonization::kinds::workable::{TypeData, TypeDataTrait};
use crate::internals::parser::ast::Representation;
use crate::internals::parser::span::{Span, Spanner};

/// Item is a core structure used to represent the AST
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Item<'temp, 'input: 'temp> {
    data: Representation<'temp, 'input>,
    type_info: TypeData,
}

impl<'temp, 'input: 'temp> Item<'temp, 'input> {
    pub fn new<T>(arg: T) -> Item<'temp, 'input>
    where
        Representation<'temp, 'input>: From<T>,
    {
        let data = Representation::from(arg);
        let type_info = TypeData::default();
        // TODO recover type info
        Self { data, type_info }
    }
}

impl<'temp, 'input: 'temp> AsRef<TypeData> for Item<'temp, 'input> {
    #[inline(always)]
    fn as_ref<'a>(&'a self) -> &'a TypeData {
        &self.type_info
    }
}

impl<'temp, 'input: 'temp> AsMut<TypeData> for Item<'temp, 'input> {
    #[inline(always)]
    fn as_mut<'a>(&'a mut self) -> &'a mut TypeData {
        &mut self.type_info
    }
}

impl<'temp, 'input: 'temp> TypeDataTrait for Item<'temp, 'input> {}

impl<'temp, 'input: 'temp> AsRef<Span<'input>> for Item<'temp, 'input> {
    #[inline(always)]
    fn as_ref<'a>(&'a self) -> &'a Span<'input> {
        self.data.as_ref()
    }
}

impl<'temp, 'input: 'temp> Spanner<'input> for Item<'temp, 'input> {}
