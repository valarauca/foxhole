#![allow(dead_code)]

/*
 * Useful macro for handling enums
 *
 */
macro_rules! from_stuff {
    ($lt: lifetime; $Kind: ty; { $($FromKind: ty => $Variant: ident),* $(,)*}) => {
        $(
        impl<$lt> From<$FromKind> for $Kind {
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
