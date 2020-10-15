use std::error::Error;

use crate::internals::canonization::kinds::workable::{TypeData, TypeDataTrait};
use crate::internals::parser::span::Spanner;

/// A method of getting the name of a variable
pub trait GetName<'lifetime> {
    fn get_name(&self) -> &'lifetime str;
}

/// Namespace conficts
pub trait NamespaceError<'temp, 'input: 'temp>: Sized + Error {
    /// When a variable is defined multiple times within a namespace,
    /// this will be invoked giving the current defination, and the old.
    fn double_def_var<A, B>(present: &A, new: &B) -> Self
    where
        A: Spanner<'input> + GetName<'temp>,
        B: Spanner<'input> + GetName<'temp>;

    /// When a function is defined multiple times within in a namespace,
    /// this will be invoked giving the current defination, and the old.
    fn double_def_func<A, B>(present: &A, new: &B) -> Self
    where
        A: Spanner<'input> + GetName<'temp>,
        B: Spanner<'input> + GetName<'temp>;

    /// when a variable isn't found
    fn var_not_found<A>(item: &A) -> Self
    where
        A: Spanner<'input>;

    /// when a function isn't found.
    fn func_not_found<A>(item: &A) -> Self
    where
        A: Spanner<'input>;

    /// This is invoked if a conditional doesn't have boolean result.
    fn condition_not_bool<A>(item: &A) -> Self
    where
        A: Spanner<'input>;

    /// When a conditional's true & false case do not return the same type.
    fn condition_not_match<A, B, C, D, E>(
        cond: &A,
        true_case: &B,
        true_type: &C,
        false_case: &D,
        false_type: &E,
    ) -> Self
    where
        A: Spanner<'input>,
        B: Spanner<'input>,
        C: TypeDataTrait,
        D: Spanner<'input>,
        E: TypeDataTrait;
}
