
use std::sync::atomic::{Ordering,AtomicU64};

/// IDENTIFIER is used to unique id each syntax element.
/// This is accomplished by an atomic counter.
const IDENTIFIER: AtomicU64 = AtomicU64::new(0);

/// Identifier is just an ID to uniquely mark syntax elements.
#[derive(Copy,Clone,Debug,PartialEq,Eq,PartialOrd,Ord,Hash)]
pub struct Identifier {
    id: u64,
}
impl Default for Identifier {
    fn default() -> Self {
        let id = increment(&IDENTIFIER);
        Self { id }
    }
}


fn increment(arg: &AtomicU64) -> u64 {
    arg.fetch_add(1,Ordering::SeqCst)
}
