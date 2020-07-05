//! Id

use std::sync::atomic::{AtomicU64, Ordering};

/// Id is a -should be unique-
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Id {
    id: u64,
}

/// IDENTIFIER is used to unique id each syntax element.
/// This is accomplished by an atomic counter.
///
const IDENTIFIER: AtomicU64 = AtomicU64::new(0);

fn increment(arg: &AtomicU64) -> u64 {
    arg.fetch_add(1, Ordering::SeqCst)
}

impl Default for Id {
    fn default() -> Self {
        let id = increment(&IDENTIFIER);
        Self { id }
    }
}
