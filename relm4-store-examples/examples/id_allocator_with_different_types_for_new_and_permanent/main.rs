use reexport::uuid::Uuid;
use record::{TemporaryIdAllocator, UuidAllocator};

#[derive(Debug,PartialEq, Eq, Hash, Clone, Copy)]
#[allow(dead_code)]
enum IdRepresentation{
    New(Uuid),
    Permanent(usize),
}

#[derive(Debug,Clone, Copy)]
struct CustomMemoryRepresentationIdAllocator{}

impl TemporaryIdAllocator for CustomMemoryRepresentationIdAllocator {
    type Type = IdRepresentation;

    fn new_id() -> Self::Type {
        IdRepresentation::New(UuidAllocator::new_id())
    }
}

fn main() {

}