
use std::sync::atomic::{AtomicI32, Ordering};
use record::{TemporaryIdAllocator};

static ID_COUNTER: AtomicI32 = AtomicI32::new(1);

#[derive(Debug,Clone,Copy)]
struct I32Allocator {}

impl TemporaryIdAllocator for I32Allocator {
    type Type = i32;

    fn new_id() -> Self::Type {
        ID_COUNTER.fetch_update(
            Ordering::SeqCst,
            Ordering::SeqCst,
            |x| Some(x + 1)
        ).unwrap()
    }
}

fn main() {
    println!("10 id's");

    for _ in 0..10 {
        println!("Id: {}", I32Allocator::new_id());
    }
}