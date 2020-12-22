use std::any::Any;

use petgraph::{
    graph::{EdgeIndex as EdgeIdx, Graph as PetGraph, NodeIndex as NodeIdx},
    visit::EdgeRef,
    Directed, Direction,
};

pub mod traits;
pub use self::traits::{EdgeTrait, NodeTrait};

pub type Node = Box<dyn Any + 'static>;
pub type NodeIndex = NodeIdx<u32>;
pub type Edge = Box<dyn Any + 'static>;
pub type EdgeIndex = EdgeIdx<u32>;
pub type ChildLambda = Box<dyn FnOnce(&mut Graph, NodeIndex)>;

/// Handles some boilerplate of building a lambda function to insert nodes.
/// In effect this operation is always identical. It always:
/// 
/// 1. Inserts the `&N` argument,
/// 2. links the "parent" with the new `&N` argument via `E`.
///
/// The "parent" is what ever is inserted when the `&N` arg's
/// `build_from_root` is handled, yes this operation is recursive.
///
/// The generic parameters of this function are "not fun", but this
/// function saves us like 6 lines of code in well over 100 locations
/// in the code base.
///
/// Its a confusing abstraction, but a useful one
pub fn build_data_child_lambda<E>(node: &<E as EdgeTrait>::N, edge: E) -> ChildLambda
where
    E: EdgeTrait,
    <E as EdgeTrait>::N: Clone + Eq + NodeTrait,
{
    let item: <E as EdgeTrait>::N = node.clone();
    Box::new(move |graph, parent| {
        let id = graph.build_from_root(item);
        graph.add_edge(parent, id, edge);
    })
}

pub fn build_typed_child_lambda<N,E>(node: &N) -> ChildLambda
where
    E: EdgeTrait + Default,
    <E as EdgeTrait>::N: Clone + Eq + NodeTrait,
    N: AsRef<<E as EdgeTrait>::N>,
{
    let item: <E as EdgeTrait>::N = node.as_ref().clone();
    Box::new(move |graph, parent| {
        let id = graph.build_from_root(item);
        graph.add_edge(parent, id, <E as Default>::default())
    })
}


/// Top Level Graph Object.
pub struct Graph {
    data: PetGraph<Node, Edge, Directed, u32>,
}

impl Graph {
    /// manages inserting children nodes for a given node type.
    pub fn build_from_root<N>(&mut self, node: N) -> NodeIndex
    where
        N: NodeTrait + Sized + Eq + Clone,
    {
        let children = node.children();
        let node_id = node.insert(self);
        for child_node in children {
            child_node(self, node_id);
        }
        node_id
    }

    pub fn add_edge<E>(&mut self, from: NodeIndex, to: NodeIndex, edge: E)
    where
        E: EdgeTrait + Sized,
    {
        debug_assert!(self.data[to].is::<E::N>());
        let edge: Box<dyn Any + 'static> = Box::new(edge);
        self.data.add_edge(from, to, edge);
    }

    pub fn get_child_node<'a, E>(&'a self, idx: NodeIndex, edge: &E) -> Option<&'a E::N>
    where
        E: EdgeTrait,
    {
        self.get_child_indexes(idx, edge)
            .filter_map(|id| self.data[id].downcast_ref::<E::N>())
            .next()
    }

    #[inline(always)]
    fn get_child_indexes<'a, E>(
        &'a self,
        idx: NodeIndex,
        edge: &'a E,
    ) -> impl Iterator<Item = NodeIndex> + 'a
    where
        E: EdgeTrait,
    {
        self.data
            .edges_directed(idx, Direction::Outgoing)
            .filter(move |e| edge.same_edge(e.weight()))
            .map(|e| e.target())
    }

    /// Inserts a node into a graph, verifying that no other copy of that node exists in the graph.
    pub fn raw_insert_node<N>(&mut self, node: N) -> NodeIndex
    where
        N: NodeTrait + Sized + Clone + Eq,
    {
        self.data.add_node(node.generalize())
    }
}
