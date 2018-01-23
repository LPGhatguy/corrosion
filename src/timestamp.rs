use std::sync::atomic::{AtomicUsize, Ordering};

/// A monotonically-increasing value used to compare when objects entered
/// their current zone.
pub type Timestamp = usize;

lazy_static! {
    static ref LAST_TIMESTAMP: AtomicUsize = AtomicUsize::new(0);
}

/// Generate a new timestamp, which should always be larger than the last one.
pub fn get_timestamp() -> Timestamp {
    LAST_TIMESTAMP.fetch_add(1, Ordering::SeqCst)
}

#[test]
fn it_gives_increasing_numbers() {
    let a = get_timestamp();
    let b = get_timestamp();

    assert!(b > a);
}
