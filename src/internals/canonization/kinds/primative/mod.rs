pub mod int;
#[doc(no_inline)]
pub use self::int::Integer;

pub mod bool;
#[doc(no_inline)]
pub use self::bool::Boolean;

/// Prim is a basic representation of a primative value.
///
/// This is used to define higher order concepts such as
/// "is this a collection of a int/bool".
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Prim {
    Int(Integer),
    Bool(Boolean),
}
