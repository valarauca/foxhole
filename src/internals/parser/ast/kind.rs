use serde::{Deserialize, Serialize};

/// Kind is used to hold typing information
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize, Deserialize)]
pub enum Kind {
    Int,
    Bool,
    CollOfInt,
    CollOfBool,
}
