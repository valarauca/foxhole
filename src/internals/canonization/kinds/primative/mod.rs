use serde::{Deserialize, Serialize};

pub mod int;
#[doc(no_inline)]
pub use self::int::{Integer, IntegerMutTrait, IntegerTrait};

/// Prim is a basic representation of a primative value.
///
/// This is used to define higher order concepts such as
/// "is this a collection of a int/bool".
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize, Deserialize)]
pub enum Prim {
    Int(Integer),
    Boolean,
}

impl Prim {
    pub fn new_int_constant(constant: i64) -> Self {
        Self::from(constant)
    }

    pub fn new_boolean() -> Self {
        Self::Boolean
    }

    pub fn new_int_dynamic<Max, Min, Const>(max: Max, min: Min, con: Const) -> Self
    where
        Max: Into<Option<i64>>,
        Min: Into<Option<i64>>,
        Const: Into<Option<i64>>,
    {
        Self::from(Integer::new(max, min, con))
    }

    pub fn new_idk_int() -> Self {
        Self::from(Integer::new(None, None, None))
    }
}

impl From<i64> for Prim {
    fn from(arg: i64) -> Self {
        Prim::Int(Integer::new_constant(arg))
    }
}

impl From<Integer> for Prim {
    fn from(arg: Integer) -> Self {
        Prim::Int(arg)
    }
}

impl AsRef<Prim> for Prim {
    #[inline(always)]
    fn as_ref<'a>(&'a self) -> &'a Prim {
        self
    }
}

impl AsMut<Prim> for Prim {
    #[inline(always)]
    fn as_mut<'a>(&'a mut self) -> &'a mut Prim {
        self
    }
}

/// Accessors on a Prim type
pub trait PrimativeTrait: AsRef<Prim> {
    /// is this a boolean?
    fn is_bool(&self) -> bool {
        self.get_int().is_none()
    }

    /// is this an integer?
    fn is_int(&self) -> bool {
        self.get_int().is_some()
    }

    /// return a readable view of the integer
    fn get_int<'a>(&'a self) -> Option<&'a Integer> {
        match self.as_ref() {
            &Prim::Int(ref int) => Some(int),
            _ => None,
        }
    }
}

/// For mutating the contents of this type
pub trait PrimativeMutTrait: AsMut<Prim> + AsRef<Prim> + PrimativeTrait {
    /// return the mutable integer data
    fn get_mut_int<'a>(&'a mut self) -> Option<&'a mut Integer> {
        match self.as_mut() {
            &mut Prim::Int(ref mut int) => Some(int),
            _ => None,
        }
    }

    /// convert this too a boolean
    fn change_to_bool(&mut self) {
        if self.is_bool() {
            return;
        }
        std::mem::replace(self.as_mut(), Prim::Boolean);
    }

    /// convert to a constant int
    fn change_to_constant_int(&mut self, constant: i64) {
        if self.is_bool() {
            let _ = std::mem::replace(self.as_mut(), Prim::from(constant));
        } else {
            match self.get_mut_int() {
                Option::None => {
                    panic!("type is a boolean, but mutable int doesn't exist?");
                }
                Option::Some(ref mut arg) => {
                    arg.set_constant(constant);
                }
            };
        }
    }
}

impl PrimativeTrait for Prim {}

impl PrimativeMutTrait for Prim {}
