
use std::any::Any;
use petgraph::graph::{Graph as PetGraph};
use petgraph::Directed;

use super::{Edge,Node,NodeIndex,Graph,ChildLambda};


pub trait EdgeTrait: Eq + Sized + 'static + Any {
    type N: 'static;

    /// checks if a graph edge is the same type, and equal to "this" edge.
    fn same_edge(&self, other: &Edge) -> bool {
        other.downcast_ref::<Self>()
            .map(|x| self.eq(x))
            .unwrap_or_else(|| false)
    }
}

pub trait NodeTrait: 'static + Any {

    /// convert self into general node type
    fn generalize(self) -> Node
    where
        Self: Sized + Eq,
    {
        Box::new(self)
    }

    fn children(&self) -> Vec<ChildLambda> {
        Vec::new()
    }

    fn insert(self, graph: &mut Graph) -> NodeIndex
    where
        Self: Sized + Eq + Clone,
    {
        graph.raw_insert_node(self)
    }

    fn same_node(&self, other: &Node) -> bool
    where
        Self: Sized + Eq + Clone,
    {
        other.downcast_ref::<Self>()
            .map(|x| self.eq(x))
            .unwrap_or_else(|| false)
    }

}

