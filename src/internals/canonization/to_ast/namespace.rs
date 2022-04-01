
use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::internals::{
    canonization::to_ast::{
        identifier::Hash,
        validation_errors::ValidationErrors,
    },
    parser::{
        span::{Span,Spanner},
        ast::{
            assign::Assign,
            func::FunctionDec,
            comparg::CompositionalFunction,
        },
    }
};

/// A simple wrapper for the "two" kinds of vars
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum VarType {
    Var(Assign),
    Const(Assign),
}
impl From<Assign> for VarType {
    fn from(x: Assign) -> Self {
        if x.is_constant {
            Self::Const(x)
        } else {
            Self::Var(x)
        }
    }
}

impl AsRef<Span> for VarType {
    fn as_ref(&self) -> &Span {
        match self {
            &VarType::Var(ref x) => x.as_ref(),
            &VarType::Const(ref x) => x.as_ref(),
        }
    }
}

impl Spanner for VarType { }

/// A simple wrapper for the "two" kinds of functions
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum FuncType {
    Normal(FunctionDec),
    Comp(CompositionalFunction),
}

impl From<FunctionDec> for FuncType {
    fn from(x: FunctionDec) -> Self { Self::Normal(x) }
}
impl From<CompositionalFunction> for FuncType {
    fn from(x: CompositionalFunction) -> Self { Self::Comp(x) }
}

impl AsRef<Span> for FuncType {
    fn as_ref(&self) -> &Span {
        match self {
            &FuncType::Normal(ref x) => x.as_ref(),
            &FuncType::Comp(ref x) => x.as_ref(),
        }
    }
}

impl Spanner for FuncType { }

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Hash, Ord, Serialize, Deserialize)]
pub struct Namespace {
    vars: BTreeMap<Hash,VarType>,
    funcs: BTreeMap<Hash,FuncType>,
}

impl Namespace {

    // TODO: Add var

    fn add_func<'a, T,E>(&mut self, arg: &'a T) -> Result<(),E>
    where
        T: 'static + Clone,
        E: ValidationErrors,
        FuncType: From<T>,
        Hash: From<&'a T>,
    {
        let hash = Hash::from(arg);
        let data = FuncType::from(<T as Clone>::clone(arg));
        match self.funcs.get(&hash) {
            Option::Some(def) => {
                return Err(E::func_conflict(&data, def));
            },
            _ => { }
        };
        self.funcs.insert(hash, data); 
        Ok(())
    }
}

