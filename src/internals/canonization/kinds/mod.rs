use serde::{Deserialize, Serialize};

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
pub use self::compositional::{Compositional, CompositionalTrait};

pub mod workable;
