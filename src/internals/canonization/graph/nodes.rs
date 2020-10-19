use serde::{Deserialize, Serialize};

use crate::internals::parser::ast::Representation;

#[derive(Clone, PartialEq, Eq, Hash, Debug, Deserialize, Serialize)]
pub enum Nodes {
    AstData(Representation),
}
