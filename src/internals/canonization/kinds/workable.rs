use serde::{Deserialize, Serialize};

use super::{Collection, CollectionTrait, Compositional, Function, Prim};
use crate::internals::parser::ast::args::FunctionArg;
use crate::internals::parser::ast::comparg::CompositionalFunction;
use crate::internals::parser::ast::func::FunctionDec;
use crate::internals::parser::ast::kind::Kind as AstKind;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize, Deserialize)]
pub enum TypeData {
    None,
    Coll(Collection),
    Prim(Prim),
    Func(Function),
    Comp(Compositional),
}

impl Default for TypeData {
    fn default() -> Self {
        Self::None
    }
}

impl AsRef<TypeData> for TypeData {
    #[inline(always)]
    fn as_ref<'a>(&'a self) -> &'a TypeData {
        self
    }
}

impl AsMut<TypeData> for TypeData {
    #[inline(always)]
    fn as_mut<'a>(&'a mut self) -> &'a mut TypeData {
        self
    }
}

impl From<Collection> for TypeData {
    fn from(arg: Collection) -> Self {
        Self::Coll(arg)
    }
}

impl From<Prim> for TypeData {
    fn from(arg: Prim) -> Self {
        Self::Prim(arg)
    }
}

impl From<Function> for TypeData {
    fn from(arg: Function) -> Self {
        Self::Func(arg)
    }
}

impl From<Compositional> for TypeData {
    fn from(arg: Compositional) -> Self {
        Self::Comp(arg)
    }
}

impl From<AstKind> for TypeData {
    fn from(arg: AstKind) -> Self {
        match arg {
            AstKind::Int => Self::Prim(Prim::new_idk_int()),
            AstKind::Bool => Self::Prim(Prim::new_boolean()),
            AstKind::CollOfInt => {
                Self::Coll(Collection::new(Prim::new_idk_int(), None, None, None))
            }
            AstKind::CollOfBool => {
                Self::Coll(Collection::new(Prim::new_boolean(), None, None, None))
            }
        }
    }
}
impl<'a> From<&'a Option<AstKind>> for TypeData {
    fn from(arg: &'a Option<AstKind>) -> Self {
        match *arg {
            Option::None => Self::None,
            Option::Some(ast_kind) => TypeData::from(ast_kind),
        }
    }
}

impl<'temp> From<&'temp AstKind> for TypeData {
    fn from(arg: &'temp AstKind) -> Self {
        Self::from(*arg)
    }
}

impl<'temp> From<&'temp Box<AstKind>> for TypeData {
    fn from(arg: &'temp Box<AstKind>) -> Self {
        Self::from(arg.as_ref())
    }
}

impl<'temp> From<&'temp Box<Option<AstKind>>> for TypeData {
    fn from(arg: &'temp Box<Option<AstKind>>) -> Self {
        match arg.as_ref() {
            &Option::None => Self::None,
            &Option::Some(ref ast_kind) => Self::from(ast_kind),
        }
    }
}

impl<'temp> From<&'temp FunctionArg> for TypeData {
    fn from(arg: &'temp FunctionArg) -> Self {
        Self::from(arg.kind.as_ref())
    }
}

impl From<&CompositionalFunction> for TypeData {
    fn from(arg: &CompositionalFunction) -> Self {
        Self::Comp(Compositional::from(arg))
    }
}

impl From<&FunctionDec> for TypeData {
    fn from(arg: &FunctionDec) -> Self {
        Self::Func(Function::from(arg))
    }
}

impl<'temp> From<&'temp Box<FunctionArg>> for TypeData {
    fn from(arg: &'temp Box<FunctionArg>) -> Self {
        Self::from(arg.kind.as_ref())
    }
}

/// Trait that encapsulates a lot of actions on the type system
pub trait TypeDataTrait: AsRef<TypeData> + AsMut<TypeData> {
    fn is_none(&self) -> bool {
        match self.as_ref() {
            &TypeData::None => true,
            _ => false,
        }
    }

    /// asserts:
    ///
    /// 1. `self` is a primative, and `other` is a collection of the same primative.
    /// 2. `self` is a collection of the same primative as `other`.
    fn is_coll_of<T: TypeDataTrait>(&self, other: &T) -> bool {
        Option::None
            .into_iter()
            .chain(
                self.get_coll()
                    .into_iter()
                    .map(|coll| coll.get_interior())
                    .zip(other.get_prim())
                    .map(|(s, o)| s == o),
            )
            .chain(
                other
                    .get_coll()
                    .into_iter()
                    .map(|coll| coll.get_interior())
                    .zip(self.get_prim())
                    .map(|(s, o)| s == o),
            )
            .next()
            .unwrap_or_else(|| false)
    }

    fn is_coll(&self) -> bool {
        self.get_coll().is_some()
    }

    fn is_prim(&self) -> bool {
        self.get_prim().is_some()
    }

    fn is_func(&self) -> bool {
        self.get_func().is_some()
    }

    fn is_comp(&self) -> bool {
        self.get_comp().is_some()
    }

    /*
     * Getter Methods
     *
     */

    fn get_coll<'a>(&'a self) -> Option<&'a Collection> {
        match self.as_ref() {
            &TypeData::Coll(ref coll) => Some(coll),
            _ => None,
        }
    }

    fn get_prim<'a>(&'a self) -> Option<&'a Prim> {
        match self.as_ref() {
            &TypeData::Prim(ref a) => Some(a),
            _ => None,
        }
    }

    fn get_func<'a>(&'a self) -> Option<&'a Function> {
        match self.as_ref() {
            &TypeData::Func(ref a) => Some(a),
            _ => None,
        }
    }

    fn get_comp<'a>(&'a self) -> Option<&'a Compositional> {
        match self.as_ref() {
            &TypeData::Comp(ref a) => Some(a),
            _ => None,
        }
    }

    /*
     * Mutation Methods
     *
     */

    /// set the value of the type based on the input argument
    /// panics if a value already exists for this type
    fn set<T>(&mut self, arg: T)
    where
        TypeData: From<T>,
    {
        if !self.is_none() {
            panic!("overriding type that already exists");
        }
        let _ = std::mem::replace(self.as_mut(), TypeData::from(arg));
    }

    fn get_mut_coll<'a>(&'a mut self) -> Option<&'a mut Collection> {
        match self.as_mut() {
            &mut TypeData::Coll(ref mut coll) => Some(coll),
            _ => None,
        }
    }

    fn get_mut_prim<'a>(&'a mut self) -> Option<&'a mut Prim> {
        match self.as_mut() {
            &mut TypeData::Prim(ref mut a) => Some(a),
            _ => None,
        }
    }

    fn get_mut_func<'a>(&'a mut self) -> Option<&'a mut Function> {
        match self.as_mut() {
            &mut TypeData::Func(ref mut a) => Some(a),
            _ => None,
        }
    }

    fn get_mut_comp<'a>(&'a mut self) -> Option<&'a mut Compositional> {
        match self.as_mut() {
            &mut TypeData::Comp(ref mut a) => Some(a),
            _ => None,
        }
    }
}

impl TypeDataTrait for TypeData {}
