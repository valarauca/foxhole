use crate::internals::{
    canonization::graph::{EdgeTrait, Graph, NodeIndex, NodeTrait},
    parser::{generated::parse_code, traits::SyntaxError},
};

/// Constructs the initial graph from source code
///
/// On success returns the `NodeIndex` that points to the 'Body' object
pub fn build_graph<E>(source: &str) -> Result<(NodeIndex, Graph), Vec<E>>
where
    E: SyntaxError,
{
    let body = parse_code::<E>(source)?;

    let mut graph = Graph::default();
    let body = graph.build_from_root(body);
    Ok((body, graph))
}
