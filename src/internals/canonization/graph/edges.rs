use serde::{Deserialize, Serialize};

pub const DOCFLOW: Edge = Edge::DocFlow;
pub const NAMESPACE: Edge = Edge::Namespace;
pub const DATAFLOW: Edge = Edge::DataFlow;

/// Edges represents the various kinds of relationships code can have with itself.
#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize, Deserialize)]
pub enum Edge {
    DocFlow = 0,
    Namespace = 1,
    DataFlow = 2,
}
