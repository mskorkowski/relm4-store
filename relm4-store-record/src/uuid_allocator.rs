use reexport::uuid;

use uuid::Uuid;

use crate::TemporaryIdAllocator;

/// Allocator for uuid
#[derive(Debug)]
pub struct UuidAllocator{}

impl TemporaryIdAllocator for UuidAllocator {
    type Type = Uuid;

    fn new_id() -> Self::Type {
        Uuid::new_v4()
    }
}