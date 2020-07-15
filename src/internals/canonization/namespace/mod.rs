
use std::collections::HashMap;

use serde::{Serialize,Deserialize};
use petgraph::graph::Graph;

use crate::internals::parser::ast::statement::{State,Statement};
use crate::internals::parser::id::{Id};

/// Namespace represents all of the data contained within a namespace
/// 
/// It demostrates how/if data is related.
pub struct Namespace<'input> {
    pool: HashMap<Id,Statement<'input>>,
    namespace: Graph<Id, i32>,
    dataflow: Graph<Id,i32>,
}
impl<'input> Namespace<'input> {

    /// allows for building a namespace
    pub fn build<I>(arg: I) -> Self
    where
        I: IntoIterator<Statement<'input>>,
    {
        let mut pool = HashMap::new();
        let mut namespace = Graph::directed();
        let mut dataflow = Graph::directed();
        let mut item = Self { pool, namespace, dataflow };
        item.build_recursive(arg);
        item
    }

    fn build_recursive<I>(&mut self, arg: I)
    where
        I: IntoIterator<Statement<'input>>,
    {
        for item in arg {

        }
    }

}
