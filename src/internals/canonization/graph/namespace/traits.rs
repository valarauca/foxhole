use std::error::Error;

use crate::internals::parser::span::Spanner;

/// A method of getting the name of a variable
pub trait GetName<'lifetime> {
    fn get_name(&self) -> &'lifetime str;
}

/// Namespace conficts
pub trait NamespaceError<'temp, 'input: 'temp>: Sized + Error {
    fn double_def_var<A, B>(present: &A, new: &B) -> Self
    where
        A: Spanner<'input> + GetName<'temp>,
        B: Spanner<'input> + GetName<'temp>;

    fn double_def_func<A, B>(present: &A, new: &B) -> Self
    where
        A: Spanner<'input> + GetName<'temp>,
        B: Spanner<'input> + GetName<'temp>;

    fn name_not_found<A>(item: &A) -> Self
    where
        A: Spanner<'input> + GetName<'temp>;
}
