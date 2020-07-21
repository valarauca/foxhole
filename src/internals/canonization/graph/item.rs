use serde::{Deserialize, Serialize};

use crate::internals::canonization::kinds::workable::{TypeData, TypeDataTrait};
use crate::internals::parser::ast::statement::{State, Statement};
use crate::internals::parser::span::{Span, Spanner};

/// Item is a core structure used to represent the AST
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Item<'temp, 'input: 'temp> {
    statement: &'temp Statement<'input>,
    type_info: TypeData,
}

impl<'temp, 'input: 'temp> From<&'temp Statement<'input>> for Item<'temp, 'input> {
    fn from(statement: &'temp Statement<'input>) -> Self {
        let type_info = match statement.as_ref() {
            &State::Declaration(ref assign) => TypeData::from(&assign.kind),
            &State::Func(ref func) => TypeData::from(func.as_ref()),
            _ => {
                // everything else needs more data
                TypeData::default()
            }
        };
        Item {
            statement,
            type_info,
        }
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
        self.statement.as_ref()
    }
}

impl<'temp, 'input: 'temp> Spanner<'input> for Item<'temp, 'input> {}
