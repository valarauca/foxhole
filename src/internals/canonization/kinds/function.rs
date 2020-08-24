use serde::{Deserialize, Serialize};

use crate::internals::parser::ast::args::FunctionArg;
use crate::internals::parser::ast::func::FunctionDec;

use super::workable::TypeData;

macro_rules! implement_index {
    (
        Type: $TypeName: ident;
        Field: $field_name: ident;
        Output: $OutputName: ident;
        Args: { $($Kind: ty),* $(,)* };
    ) => {

        $(
            impl std::ops::Index<$Kind> for $TypeName {
                type Output = $OutputName;
                #[inline(always)]
                fn index<'a>(&'a self, arg: $Kind) -> &'a Self::Output {
                    &self.$field_name[arg as usize]
                }
            }
            impl std::ops::IndexMut<$Kind> for $TypeName {
                #[inline(always)]
                fn index_mut<'a>(&'a mut self, arg: $Kind) -> &'a mut Self::Output {
                    &mut self.$field_name[arg as usize]
                }
            }
        )*
    };
}

/// Function encodes information about functions
/// non-homomorphic-functions.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize, Deserialize)]
pub struct Function {
    args: Box<[TypeData]>,
    ret: Box<TypeData>,
}

impl Function {
    /// allows for building new functions
    /// required for compositional functions
    #[inline(always)]
    pub fn new<I, T, A>(args: I, ret: A) -> Self
    where
        TypeData: From<T>,
        I: IntoIterator<Item = T>,
        TypeData: From<A>,
    {
        let args = args
            .into_iter()
            .map(<TypeData as From<T>>::from)
            .collect::<Vec<TypeData>>()
            .into_boxed_slice();
        let ret = Box::new(TypeData::from(ret));
        Self { args, ret }
    }
}

impl<'temp, 'input: 'temp> From<&'temp FunctionDec<'input>> for Function {
    fn from(arg: &'temp FunctionDec) -> Function {
        Function::new(&arg.args, arg.ret.as_ref())
    }
}

implement_index! {
    Type: Function;
    Field: args;
    Output: TypeData;
    Args: { u8, u16, u32, u64, usize };
}

impl AsRef<Function> for Function {
    #[inline(always)]
    fn as_ref<'a>(&'a self) -> &'a Function {
        self
    }
}

impl AsMut<Function> for Function {
    #[inline(always)]
    fn as_mut<'a>(&'a mut self) -> &'a mut Function {
        self
    }
}

/// accessor methods for functions
pub trait FunctionTrait: AsRef<Function> + std::ops::Index<usize, Output = TypeData> {
    /// how many arguments are there
    fn args_len(&self) -> usize {
        self.as_ref().args.len()
    }

    /// fetches the functions return kind
    fn get_return<'a>(&'a self) -> &'a TypeData {
        &self.as_ref().ret
    }
}

impl FunctionTrait for Function {}

pub trait FunctionMutTrait: AsMut<Function> + FunctionTrait + std::ops::IndexMut<usize> {
    /// get return argument, but mutable
    fn get_mut_return<'a>(&'a mut self) -> &'a mut TypeData {
        &mut self.as_mut().ret
    }
}

impl FunctionMutTrait for Function {}
