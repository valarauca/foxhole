use serde::{Deserialize, Serialize};

use super::workable::TypeData;
use super::Collection;
use super::{Function, FunctionTrait};
use crate::internals::parser::ast::comparg::CompositionalFunction;

/// Compositional represnts a compositional function, or
/// a homo-morphism
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize, Deserialize)]
pub struct Compositional {
    identity: Function,
    primative: Function,
    collection: Function,
}

impl AsRef<Compositional> for Compositional {
    #[inline(always)]
    fn as_ref<'a>(&'a self) -> &'a Self {
        self
    }
}

impl AsMut<Compositional> for Compositional {
    #[inline(always)]
    fn as_mut<'a>(&'a mut self) -> &'a mut Self {
        self
    }
}

/// Non-mutable things you can do with a compositional function
pub trait CompositionalTrait: AsRef<Compositional> {
    fn get_identity<'a>(&'a self) -> &'a Function {
        &self.as_ref().identity
    }

    fn get_primative<'a>(&'a self) -> &'a Function {
        &self.as_ref().primative
    }

    fn get_collection<'a>(&'a self) -> &'a Function {
        &self.as_ref().collection
    }
}

impl From<&CompositionalFunction> for Compositional {
    /// NOTE:
    ///       This does not verify that the internal function
    ///       is of the correct type.
    ///       It generates internal arguments based on the assumption
    ///       the return type is correct.
    /// NOTE:
    ///       This doesn't like collections
    /// TODO:
    ///       Type Checking must be performed after namespace validation.
    fn from(arg: &CompositionalFunction) -> Self {
        let (ret, coll) = match TypeData::from(arg.ret.as_ref()) {
            TypeData::Prim(x) => {
                let prim = TypeData::Prim(x.clone());
                let coll = TypeData::from(Collection::new(x, None, None, None));
                (prim, coll)
            }
            anything_else => {
                panic!(
                    "compositional functions cannot return type: {:?}",
                    anything_else
                );
            }
        };
        Self {
            identity: Function::new(Option::<TypeData>::None, ret.clone()),
            primative: Function::new(Some(ret.clone()), ret.clone()),
            collection: Function::new(Some(coll), ret),
        }
    }
}
