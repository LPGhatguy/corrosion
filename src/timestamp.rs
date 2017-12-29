use std::sync::atomic::{AtomicUsize, Ordering};

pub type Timestamp = usize;

lazy_static! {
    static ref LAST_TIMESTAMP: AtomicUsize = AtomicUsize::new(0);
}

pub fn get_timestamp() -> Timestamp {
    LAST_TIMESTAMP.fetch_add(1, Ordering::SeqCst)
}

#[test]
fn it_gives_unique_numbers() {
    let a = get_timestamp();
    let b = get_timestamp();

    assert!(a != b);
}
