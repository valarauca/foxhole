#![allow(dead_code)]

/*
 * Useful macro for handling enums
 *
 */
macro_rules! stuff {
    (
        Name: $Kind: ident;
        Trait: $TraitName: ident;
        Lifetime: $lt: lifetime;
        From: { $($FromKind: ty => $Variant: ident => $is_a: ident => $get_a: ident),* $(,)*}) => {

        #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
        pub enum $Kind<$lt> {
            $(
                #[serde(borrow)] $Variant(Box<$FromKind>),
            )*
        }

        impl<$lt> AsRef<$Kind<$lt>> for $Kind<$lt> {
            fn as_ref<'a>(&'a self) -> &'a Self {
                self
            }
        }

        pub trait $TraitName<$lt>: AsRef<$Kind<$lt>> {
            $(
                fn $is_a(&self) -> bool {
                    match self.as_ref() {
                        &$Kind::$Variant(_) => true,
                        _ => false
                    }
                }

                fn $get_a<'a>(&'a self) -> Option<&'a $FromKind> {
                    let x: &'a $Kind<$lt> = self.as_ref();
                    match x {
                        &$Kind::$Variant(ref a) => Some(a.as_ref()),
                        _ => None
                    }
                }
            )*
        }

        impl<$lt> $TraitName<$lt> for $Kind<$lt> { }

        $(
        impl<$lt> From<$FromKind> for $Kind<$lt> {
            #[inline(always)]
            fn from(arg: $FromKind) -> Self {
                Self::$Variant(Box::new(arg))
            }
        }
        )*
    };
}

pub mod args;
pub mod assign;
pub mod comparg;
pub mod condition;
pub mod expr;
pub mod func;
pub mod ident;
pub mod invoke;
pub mod kind;
pub mod op;
pub mod statement;
pub mod template;

use crate::internals::parser::ast::args::FunctionArg;
use crate::internals::parser::ast::assign::Assign;
use crate::internals::parser::ast::comparg::{CompositionalFunction, CompositionalFunctionArg};
use crate::internals::parser::ast::condition::Conditional;
use crate::internals::parser::ast::expr::Expression;
use crate::internals::parser::ast::func::FunctionDec;
use crate::internals::parser::ast::ident::Ident;
use crate::internals::parser::ast::invoke::Invoke;
use crate::internals::parser::ast::op::Operation;
use crate::internals::parser::ast::statement::{State, Statement};
use crate::internals::parser::ast::template::Template;
use crate::internals::parser::span::{Span, Spanner};

/// Representation is all possible values of
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Representation<'temp, 'input: 'temp> {
    Statement(&'temp Statement<'input>),
    Ident(&'temp Ident<'input>),
    Assign(&'temp Assign<'input>),
    FunctionArg(&'temp FunctionArg<'input>),
    Template(&'temp Template<'input>),
    CompositionalFunctionArg(&'temp CompositionalFunctionArg<'input>),
    CompositionalFunction(&'temp CompositionalFunction<'input>),
    Conditional(&'temp Conditional<'input>),
    Expression(&'temp Expression<'input>),
    FunctionDec(&'temp FunctionDec<'input>),
    Invoke(&'temp Invoke<'input>),
    Operation(&'temp Operation<'input>),
}

impl<'temp, 'input: 'temp> From<&'temp Statement<'input>> for Representation<'temp, 'input> {
    fn from(arg: &'temp Statement<'input>) -> Self {
        Self::Statement(arg)
    }
}

impl<'temp, 'input: 'temp> From<&'temp Ident<'input>> for Representation<'temp, 'input> {
    fn from(arg: &'temp Ident<'input>) -> Self {
        Self::Ident(arg)
    }
}

impl<'temp, 'input: 'temp> From<&'temp Assign<'input>> for Representation<'temp, 'input> {
    fn from(arg: &'temp Assign<'input>) -> Self {
        Self::Assign(arg)
    }
}

impl<'temp, 'input: 'temp> From<&'temp FunctionArg<'input>> for Representation<'temp, 'input> {
    fn from(arg: &'temp FunctionArg<'input>) -> Self {
        Self::FunctionArg(arg)
    }
}

impl<'temp, 'input: 'temp> From<&'temp Template<'input>> for Representation<'temp, 'input> {
    fn from(arg: &'temp Template<'input>) -> Self {
        Self::Template(arg)
    }
}

impl<'temp, 'input: 'temp> From<&'temp CompositionalFunctionArg<'input>>
    for Representation<'temp, 'input>
{
    fn from(arg: &'temp CompositionalFunctionArg<'input>) -> Self {
        Self::CompositionalFunctionArg(arg)
    }
}

impl<'temp, 'input: 'temp> From<&'temp CompositionalFunction<'input>>
    for Representation<'temp, 'input>
{
    fn from(arg: &'temp CompositionalFunction<'input>) -> Self {
        Self::CompositionalFunction(arg)
    }
}

impl<'temp, 'input: 'temp> From<&'temp Conditional<'input>> for Representation<'temp, 'input> {
    fn from(arg: &'temp Conditional<'input>) -> Self {
        Self::Conditional(arg)
    }
}

impl<'temp, 'input: 'temp> From<&'temp Expression<'input>> for Representation<'temp, 'input> {
    fn from(arg: &'temp Expression<'input>) -> Self {
        Self::Expression(arg)
    }
}

impl<'temp, 'input: 'temp> From<&'temp FunctionDec<'input>> for Representation<'temp, 'input> {
    fn from(arg: &'temp FunctionDec<'input>) -> Self {
        Self::FunctionDec(arg)
    }
}

impl<'temp, 'input: 'temp> From<&'temp Invoke<'input>> for Representation<'temp, 'input> {
    fn from(arg: &'temp Invoke<'input>) -> Self {
        Self::Invoke(arg)
    }
}

impl<'temp, 'input: 'temp> From<&'temp Operation<'input>> for Representation<'temp, 'input> {
    fn from(arg: &'temp Operation<'input>) -> Self {
        Self::Operation(arg)
    }
}

impl<'temp, 'input: 'temp> AsRef<Span<'input>> for Representation<'temp, 'input> {
    fn as_ref<'a>(&'a self) -> &'a Span<'input> {
        match self {
            &Self::Statement(ref a) => a.as_ref(),
            &Self::Ident(ref a) => a.as_ref(),
            &Self::Assign(ref a) => a.as_ref(),
            &Self::FunctionArg(ref a) => a.as_ref(),
            &Self::Template(ref a) => a.as_ref(),
            &Self::CompositionalFunctionArg(ref a) => a.as_ref(),
            &Self::CompositionalFunction(ref a) => a.as_ref(),
            &Self::Conditional(ref a) => a.as_ref(),
            &Self::Expression(ref a) => a.as_ref(),
            &Self::FunctionDec(ref a) => a.as_ref(),
            &Self::Invoke(ref a) => a.as_ref(),
            &Self::Operation(ref a) => a.as_ref(),
        }
    }
}

impl<'temp, 'input: 'temp> Spanner<'input> for Representation<'temp, 'input> {}
