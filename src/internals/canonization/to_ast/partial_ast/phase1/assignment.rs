use serde::{Deserialize, Serialize};

use crate::internals::{
    parser::{
        span::{Span,Spanner},
    },
    canonization::{
        to_ast::{
            validation_errors::ValidationErrors,
            partial_ast::phase1::expr::P1Expression,
        },
        kinds::{ workable::{TypeDataTrait,TypeData} },
        
    },
};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct P1Assign {
    pub span: Box<Span>,
    pub identifier: Hash,
    pub kind: Box<TypeData>,
    pub expr: P1Expression,
}


