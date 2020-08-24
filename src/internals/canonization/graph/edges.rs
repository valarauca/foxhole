use serde::{Deserialize, Serialize};

/// Edges represents the various kinds of relationships code can have with itself.
#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize, Deserialize)]
pub enum Edge {
    Namespace = 1,
    DataFlow = 2,
}
