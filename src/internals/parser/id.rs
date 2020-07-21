use std::sync::atomic::{spin_loop_hint, AtomicU64, Ordering};

use serde::{Deserialize, Serialize};

/// Id is a -should be unique-
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Id {
    id: u64,
}
impl Id {
    /// Sets the global ID value based on the highest value found while deserializing
    pub(in crate::internals::parser) fn set_id(&self) {
        let mut curr = 0;
        loop {
            curr = IDENTIFIER.load(Ordering::SeqCst);
            if self.id > curr {
                curr = IDENTIFIER.compare_and_swap(curr, self.id, Ordering::SeqCst);
            }
            if self.id <= curr {
                return;
            }
            spin_loop_hint();
        }
    }
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
