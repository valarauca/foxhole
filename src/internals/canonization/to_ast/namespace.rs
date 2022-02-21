
use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::internals::{
    canonization::to_ast::identifier::Hash,
    parser::ast::{
        assign::Assign,
        func::FunctionDec,
        comparg::CompositionalFunction,
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

/// Handles providing an error when namespace conflicts occur
pub trait NamespaceError: Sized {

    fn var_conflict(new: &VarType, old: &VarType) -> Self;

    fn func_conflict(new: &FuncType, old: &FuncType) -> Self;
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Hash, Ord, Serialize, Deserialize)]
pub struct Namespace {
    vars: BTreeMap<Hash,VarType>,
    funcs: BTreeMap<Hash,FuncType>,
}

impl Namespace {

    fn add_func<'a, T,E>(&mut self, arg: &'a T) -> Result<(),E>
    where
        T: 'static + Clone,
        E: NamespaceError,
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

