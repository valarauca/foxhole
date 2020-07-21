use serde::{Deserialize, Serialize};

use super::{Function, FunctionTrait};

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
