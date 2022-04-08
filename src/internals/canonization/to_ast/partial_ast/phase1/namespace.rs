use serde::{Deserialize, Serialize};

use crate::internals::canonization::to_ast::identifier::Hash;


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Identifier {
    span: Box<Span>,
    constant: bool,
    hash_stack: Vec<Hash>,
}
