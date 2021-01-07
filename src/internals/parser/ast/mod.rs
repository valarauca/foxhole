#![allow(dead_code)]

use serde::{Deserialize, Serialize};

/*
 * Useful macro for handling enums
 *
 */
macro_rules! stuff {
    (
        Name: $Kind: ident;
        Trait: $TraitName: ident;
        From: { $($FromKind: ty => $Variant: ident => $is_a: ident => $get_a: ident),* $(,)*}) => {

        #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
        pub enum $Kind {
            $(
                 $Variant(Box<$FromKind>),
            )*
        }

        impl AsRef<$Kind> for $Kind {
            fn as_ref<'a>(&'a self) -> &'a Self {
                self
            }
        }

        pub trait $TraitName: AsRef<$Kind> {
            $(
                fn $is_a(&self) -> bool {
                    match self.as_ref() {
                        &$Kind::$Variant(_) => true,
                        _ => false
                    }
                }

                fn $get_a<'a>(&'a self) -> Option<&'a $FromKind> {
                    let x: &'a $Kind = self.as_ref();
                    match x {
                        &$Kind::$Variant(ref a) => Some(a.as_ref()),
                        _ => None
                    }
                }
            )*
        }

        impl $TraitName for $Kind { }

        $(
        impl From<$FromKind> for $Kind {
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
