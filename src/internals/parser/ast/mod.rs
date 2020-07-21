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

                /*
                fn $get_a<'a>(&'a self) -> Option<&'a $FromKind> {
                    let x: &'a $Kind<$lt> = self.as_ref();
                    match x {
                        &$Kind::$Variant(ref item) => Some(item),
                        _ => None
                    }
                }
                */
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
