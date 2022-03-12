
use crate::internals::{
    parser::{
        ast::op::{Op},
        span::{Spanner},
    },
    canonization::kind::workable::{TypeDataTrait,TypeData},
};


pub trait TypeError: Sized {
    fn no_type_information<S>(arg: &S) -> Self;

}


pub trait 

