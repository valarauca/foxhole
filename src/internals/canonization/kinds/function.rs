
use super::Kind;

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
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Hash)]
pub struct Function {
    args: Box<[Kind]>,
    ret: Kind,
}

impl Function {

    /// builds a new function
    #[allow(dead_code)]
    pub fn new<R,I>(ret: R, args: I) -> Self
    where
        Kind: From<R>,
        I: IntoIterator<Item=Kind>,
    {
        let args = args.into_iter().collect::<Vec<Kind>>().into_boxed_slice();
        let ret = Kind::from(ret);
        Self { args, ret }
    }
}

implement_index! {
    Type: Function;
    Field: args;
    Output: Kind;
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
pub trait FunctionTrait: AsRef<Function> + std::ops::Index<usize,Output=Kind> {

    /// how many arguments are there
    fn args_len(&self) -> usize {
        self.as_ref().args.len()
    }

    /// fetches the functions return kind
    fn get_return<'a>(&'a self) -> &'a Kind {
        & self.as_ref().ret
    }
}

impl FunctionTrait for Function { }

pub trait FunctionMutTrait: AsMut<Function> + FunctionTrait + std::ops::IndexMut<usize> {

    /// get return argument, but mutable
    fn get_mut_return<'a>(&'a mut self) -> &'a mut Kind {
        &mut self.as_mut().ret
    }
}

impl FunctionMutTrait for Function { }
