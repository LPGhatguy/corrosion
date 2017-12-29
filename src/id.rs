use std::sync::atomic::{AtomicUsize, Ordering};

pub type Id = usize;

lazy_static! {
    static ref LAST_ID: AtomicUsize = AtomicUsize::new(0);
}

pub fn get_id() -> Id {
    LAST_ID.fetch_add(1, Ordering::SeqCst)
}

#[test]
fn it_gives_unique_numbers() {
    let a = get_id();
    let b = get_id();

    assert!(a != b);
}
