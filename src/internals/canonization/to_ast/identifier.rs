use seahash::hash_seeded;
use serde::{Deserialize, Serialize};


use crate::internals::parser::{
    ast::{
        ident::Ident,
        comparg::CompositionalFunction,
        assign::Assign,
        func::FunctionDec,
    },
    span::{Spanner},
};

/// Universal reprsentation of a identifier
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Hash {
    a: u64,
    b: u64,
    c: u64,
    d: u64,
}
impl Hash {

    /// build a new hash
    fn from_span<S>(span: &S) -> Self
    where
        S: Spanner,
    {
        let b = span.get_span().as_bytes();
        let z = u64::MIN;
        let m = u64::MAX;
        Self {
            a: hash_seeded(b, z,z,z,z),
            b: hash_seeded(b, m,z,z,m),
            c: hash_seeded(b, z,m,m,z),
            d: hash_seeded(b, m,m,m,m),
        }
    }
}

/*
 * These handle extracting names
 *
 */

impl From<&Ident> for Hash {
    fn from(x: &Ident) -> Self {
        Self::from_span(x)
    }
}
impl From<&Box<Ident>> for Hash {
    fn from(x: &Box<Ident>) -> Self {
        let y: &Ident = x.as_ref();
        Hash::from(y)
    }
}

impl From<&Assign> for Hash {
    fn from(x: &Assign) -> Self {
        Hash::from(&x.name)
    }
}
impl From<&Box<Assign>> for Hash {
    fn from(x: &Box<Assign>) -> Self {
        let y: &Assign = x.as_ref();
        Hash::from(y)
    }
}

impl From<&FunctionDec> for Hash {
    fn from(x: &FunctionDec) -> Self {
        Hash::from(&x.name)
    }
}
impl From<&Box<FunctionDec>> for Hash {
    fn from(x: &Box<FunctionDec>) -> Self {
        let y: &FunctionDec = x.as_ref();
        Self::from(y)
    }
}

impl From<&CompositionalFunction> for Hash {
    fn from(x: &CompositionalFunction) -> Self {
        Self::from(&x.name)
    }
}
impl From<&Box<CompositionalFunction>> for Hash {
    fn from(x: &Box<CompositionalFunction>) -> Self {
        let y: &CompositionalFunction = x.as_ref();
        Self::from(y)
    }
}
