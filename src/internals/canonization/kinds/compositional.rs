
use super::{Kind, Prim, Function, FunctionTrait};

/// Compositional represnts a compositional function, or
/// a homo-morphism
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Hash)]
pub struct Compositional {
    identity: Function,
    primative: Function,
    collection: Function,
}

impl Compositional {

    /// Build a new compositional function
    #[allow(dead_code)]
    pub fn new<I,P,C>(identity: I, prim: P, coll: C) -> Self
    where
        Kind: From<I>,
        Function: From<P>,
        Function: From<C>,
    {
        let identity = Function::new(identity, None);
        let primative = Function::from(prim);
        let collection = Function::from(coll);

        // compositional functions require specific things
        // always be true
        debug_assert_eq!(primative.args_len(), 1);
        debug_assert_eq!(identity.args_len(), 0);
        debug_assert_eq!(collection.args_len(), 2);
        debug_assert_eq!(identity.get_return(), primative.get_return());
        debug_assert_eq!(identity.get_return(), collection.get_return());
        Self { identity, primative, collection }
    }
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
        & self.as_ref().identity
    }

    fn get_primative<'a>(&'a self) -> &'a Function {
        & self.as_ref().primative
    }

    fn get_collection<'a>(&'a self) -> &'a Function {
        & self.as_ref().collection
    }
}
