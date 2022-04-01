use serde::{Deserialize, Serialize};

use crate::internals::canonization::to_ast::identifier::Hash;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Identifier {
    span: Box<Span>,
    hash_stack: Vec<Hash>,
}

pub struct Namespace {
    id: Hash,
    vars: BTreeMap<Hash, 
}
