use std::collections::{BTreeMap, HashMap};

use petgraph::{
    graph::{Graph, NodeIndex},
    Directed,
};

use crate::internals::parser::{
    ast::assign::Assign,
    ast::Representation,
    id::Id,
    span::{Span, Spanner},
};

mod namespace;

/*
pub mod edges;
pub mod item;
pub mod traits;
use self::{
    edges::Edge,
    item::Item,
    traits::CanonizationError,
};
*/

/*
#[derive(Default)]
struct Namespace<'temp,'input: 'temp> {
    bind_site: HashMap<&'input str, &'temp Assign<'input>>,
    bind_type: HashMap<&'input str,
}
*/

/*
/// Canonization holds information about the larger graph
#[derive(Default)]
pub struct Canonization<'temp, 'input: 'temp> {
    idcoll: HashMap<&'input str, Vec<Id>>,
    data: BTreeMap<Id,Item<'temp,'input>>,
    nodes: BTreeMap<Id, NodeIndex<usize>>,
    graph: Graph<Id, Edge, Directed, usize>,
}
impl<'temp, 'input: 'temp> Canonization<'temp, 'input> {


    /*
     * These methods pertain to building the graph
     *
     */
    fn add_recursive<T>(&mut self, arg: T)
    where
        T: Spanner<'input>,
        Representation<'temp, 'input>: From<T>,
    {
        // insert the item
        let (node_id, id) = self.add_item(arg);

        // look the item's children in a manner specific to itself
        match self.get_item(id).unwrap().as_ref() {
            &Representation::Statement(ref statement) => {

                // deligate to the proper sub field
                match statement {
                    &State::Declaration(ref assign) => {
                        self.add_recursive(assign);
                    }
                    &State::Func(ref func) => {
                        self.add_recursive(func);
                    }
                    &State::CompFunc(ref comp) => {
                        self.add_recursive(comp);
                    }
                    &State::Termination(ref term) => {
                        self.add_recursive(term);
                    }
                };
            }
            &Representation::Ident(ref identifier) => {
                self.add_recursive(identifier);
            }
            &Representation::Assign(ref assign) => {
                self.add_recursive(&assign.name);
                self.add_recursive(&assign.expr);
            }
            &Representation::FunctionArg(ref func_arg) => {
                self.add_recursive(&func_arg.name);
            }
            &Representation::Template(ref template) => {
            }
            &Representation::CompositionalFunctionArg(ref arg) => {
            }
            &Representation::CompositionalFunction(ref arg) => {
            }
            &Representation::Conditional(ref conditional) => {
            }
            &Representation::Expression(ref expr) => {
            }
            &Representation::FunctionDec(ref func) => {
            }
            &Representation::Invoke(ref invoke) => {
            }
            &Representation::Operation(ref op) => {
            }
        };
    }

    /// fetches an item
    fn get_item<'a>(&'a self, id: Id) -> Option<&'a Item<'temp,'input>> {
        self.data.get(&id)
    }

    /// fetces an item's node index
    fn get_index<'a>(&'a self, id: Id) -> Option<&'a NodeIndex<usize>> {
        self.nodes.get(&id)
    }

    /// inserts a syntax element into the validation utility
    fn add_item<T>(&mut self, arg: T) -> (NodeIndex<usize>,Id)
    where
        T: Spanner<'input>,
        Representation<'temp,'input>: From<T>,
    {

        // does this already exist?
        let id = arg.get_id();
        match (self.get_item(id),self.get_index(id)) {
            (Option::Some(_),Option::Some(ref index)) => return (*index,id),
            _ => { }
        };

        let item = Item::new(arg);
        debug_assert_eq!(item.get_id(), id);
        self.data.insert(id, item.clone());
        let node_id = self.graph.add_node(id);
        self.nodes.insert(id, node_id);
        (node_id, id)
    }
}
*/
