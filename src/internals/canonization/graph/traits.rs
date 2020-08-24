use std::fmt::Display;

use crate::internals::parser::span::Spanner;

/// CanonizationError represents some fault with program flow,
/// or typing decisions.
pub trait CanonizationError<'temp, 'input: 'temp>: Sized + Display {
    /// interaction with 2 or more components
    fn build_interaction(
        msg: &'static str,
        receiver: Option<&'temp dyn Spanner<'input>>,
        sender: Option<&'temp dyn Spanner<'input>>,
    ) -> Self;

    /// simple error
    fn build(msg: &'static str, item: &'temp dyn Spanner<'input>) -> Self;
}
