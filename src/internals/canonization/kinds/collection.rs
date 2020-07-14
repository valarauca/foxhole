
#[allow(unused_imports)]
use crate::internals::canonization::kinds::primative::{Prim,PrimativeTrait,PrimativeMutTrait};
#[allow(unused_imports)]
use crate::internals::canonization::kinds::primative::int::{Integer,IntegerTrait,IntegerMutTrait};

/// Collection of primatives
#[derive(Copy,Clone,PartialEq,Eq,PartialOrd,Ord,Hash,Debug)]
pub struct Collection {
    interior: Prim,
    size: Integer,
}

impl Collection {

    /// build a new collection
    #[allow(dead_code)]
    pub fn new<Interior, MaxSize, MinSize, ConstantSize>(kind: Interior, max: MaxSize, min: MinSize, con: ConstantSize) -> Self
    where
        Prim: From<Interior>,
        MaxSize: Into<Option<i64>>,
        MinSize: Into<Option<i64>>,
        ConstantSize: Into<Option<i64>>,
    {
        Self {
            interior: Prim::from(kind),
            size: Integer::new(max,min,con),
        }
    }
}

/*
 * Boilerplate
 *
 */

impl AsRef<Collection> for Collection {
    #[inline(always)]
    fn as_ref<'a>(&'a self) -> &'a Self {
        self
    }
}

impl AsMut<Collection> for Collection {
    #[inline(always)]
    fn as_mut<'a>(&'a mut self) -> &'a mut Self {
        self
    }
}

impl AsRef<Prim> for Collection {
    #[inline(always)]
    fn as_ref<'a>(&'a self) -> &'a Prim {
        &self.interior
    }
}

impl AsMut<Prim> for Collection {
    #[inline(always)]
    fn as_mut<'a>(&'a mut self) -> &'a mut Prim {
        &mut self.interior
    }
}

impl PrimativeTrait for Collection { }

impl PrimativeMutTrait for Collection { }

/// CollectionTrait are standard accessors on a collection
pub trait CollectionTrait: AsRef<Collection> + PrimativeTrait {

    /// is the interior type a boolean
    fn contains_bool(&self) -> bool {
        <Self as PrimativeTrait>::is_bool(self)
    }

    /// is the interior type a integer
    fn contains_int(&self) -> bool {
        <Self as PrimativeTrait>::is_int(self)
    }

    /// return the interior type representation
    fn get_interior<'a>(&'a self) -> &'a Prim {
        <Self as AsRef<Prim>>::as_ref(self)
    }

    /// returns the size of the collection
    fn len<'a>(&'a self) -> &'a Integer {
        &<Self as AsRef<Collection>>::as_ref(self).size
    }
}

impl CollectionTrait for Collection { }


/// CollectionMutTrait permits mutability
pub trait CollectionMutTrait: AsMut<Collection> + CollectionTrait + PrimativeTrait + PrimativeMutTrait {

    /// return the mutable interior length if it exists
    fn mut_len<'a>(&'a mut self) -> &'a mut Integer {
        &mut <Self as AsMut<Collection>>::as_mut(self).size
    }
}

impl CollectionMutTrait for Collection { }
