#[allow(unused_imports)]
pub use self::primative::int::{Integer, IntegerMutTrait, IntegerTrait};
pub mod primative;
#[allow(unused_imports)]
pub use self::primative::{Prim, PrimativeMutTrait, PrimativeTrait};
pub mod collection;
#[allow(unused_imports)]
pub use self::collection::{Collection, CollectionMutTrait, CollectionTrait};
pub mod function;
pub use self::function::{Function, FunctionMutTrait, FunctionTrait};
#[allow(unused_imports)]
pub mod compositional;

/// Kind is a core "type" representation
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Kind {
    Coll(Collection),
    Prim(Prim),
}

impl From<Prim> for Kind {
    #[inline(always)]
    fn from(prim: Prim) -> Self {
        Kind::Prim(prim)
    }
}

impl From<Integer> for Kind {
    #[inline(always)]
    fn from(int: Integer) -> Self {
        Kind::Prim(Prim::from(int))
    }
}

impl From<Collection> for Kind {
    #[inline(always)]
    fn from(coll: Collection) -> Self {
        Kind::Coll(coll)
    }
}

impl AsRef<Kind> for Kind {
    #[inline(always)]
    fn as_ref<'a>(&'a self) -> &'a Kind {
        self
    }
}

impl AsMut<Kind> for Kind {
    #[inline(always)]
    fn as_mut<'a>(&'a mut self) -> &'a mut Kind {
        self
    }
}

impl AsRef<Prim> for Kind {
    #[inline]
    fn as_ref<'a>(&'a self) -> &'a Prim {
        match self {
            &Kind::Coll(ref coll) => coll.as_ref(),
            &Kind::Prim(ref prim) => prim,
        }
    }
}

impl AsMut<Prim> for Kind {
    #[inline]
    fn as_mut<'a>(&'a mut self) -> &'a mut Prim {
        match self {
            &mut Kind::Coll(ref mut coll) => coll.as_mut(),
            &mut Kind::Prim(ref mut prim) => prim,
        }
    }
}

impl PrimativeTrait for Kind {}

impl PrimativeMutTrait for Kind {}

/// KindTrait offers public accessors which do not mutate the interior values
pub trait KindTrait: AsRef<Kind> + PrimativeTrait {
    /// returns if this is a collection
    fn is_coll(&self) -> bool {
        match <Self as AsRef<Kind>>::as_ref(self) {
            &Kind::Coll(_) => true,
            _ => false,
        }
    }

    /// returns the collection's size information
    fn get_coll_len<'a>(&'a self) -> Option<&'a Integer> {
        match <Self as AsRef<Kind>>::as_ref(self) {
            &Kind::Coll(ref coll) => Some(coll.len()),
            _ => None,
        }
    }

    /// returns the interior integer IFF this is a collection of integers, or an integer
    fn get_int<'a>(&'a self) -> Option<&'a Integer> {
        match <Self as AsRef<Kind>>::as_ref(self) {
            &Kind::Coll(ref coll) => coll.get_int(),
            &Kind::Prim(ref prim) => prim.get_int(),
        }
    }
}

impl KindTrait for Kind {}

/// mutable methods on kinds
pub trait KindMutTrait: AsMut<Kind> + KindTrait + PrimativeMutTrait {
    /// returns a mutable reference to the interior collection's length
    fn mut_len<'a>(&'a mut self) -> Option<&'a mut Integer> {
        match <Self as AsMut<Kind>>::as_mut(self) {
            &mut Kind::Coll(ref mut coll) => Some(coll.mut_len()),
            _ => None,
        }
    }

    /// return a mutable integer reference IFF this is a collection of integers, or an integer
    fn get_mut_int<'a>(&'a mut self) -> Option<&'a mut Integer> {
        match <Self as AsMut<Kind>>::as_mut(self) {
            &mut Kind::Coll(ref mut coll) => coll.get_mut_int(),
            &mut Kind::Prim(ref mut prim) => prim.get_mut_int(),
        }
    }
}

impl KindMutTrait for Kind {}
