use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

use record::{TemporaryIdAllocator, Id, Record, IdentityError};

#[derive(Debug,Clone,Copy)]
struct Char24IdAllocator {}

impl TemporaryIdAllocator for Char24IdAllocator {
    type Type = [char;24];

    fn new_id() -> Self::Type {
        let mut new_id: [char;24] = [' ';24];

        for (idx, c) in thread_rng()
            .sample_iter(&Alphanumeric)
            .map(char::from)
            .enumerate()
            .take(24) 
        {
            new_id[idx] = c;
        }

        new_id
    }
}

#[derive(Clone,Debug)]
struct RecordWithCustomId {
    id: Id<RecordWithCustomId>,
}

impl Record for RecordWithCustomId {
    type Allocator = Char24IdAllocator;

    fn get_id(&self) -> Id<Self> {
        self.id
    }

    fn set_permanent_id(&mut self, value: [char; 24]) -> Result<(), record::IdentityError> {
        match self.id {
            Id::New{value: _} => {
                self.id = Id::Permanent{value};
                Ok(())
            },
            Id::Permanent{ value: _} => {
                Err(IdentityError("Permanent id already set"))
            }
        }
    }
}

fn main() {

    println!("Sample id values:");
    println!("\tSample id 1: {:?}", Char24IdAllocator::new_id());
    println!("\tSample id 2: {:?}", Char24IdAllocator::new_id());
    println!("\tSample id 3: {:?}", Char24IdAllocator::new_id());

    let new_id: Id<RecordWithCustomId> = Id::New{
        value: Char24IdAllocator::new_id()
    };
    println!("New id: {:?}", new_id);
    
    // value of this id in most cases will come back from db
    //
    // in our example we can do that since temporary id and permanent id internally
    // has same representation which doesn't need to be a case if you use enums to
    // define TemporaryIdAllocator::Type value
    //
    // In your application you shouldn't write a code like that. It's a bad practice.
    // If you depend on the ability to feed the Id::Permanent with results of 
    // TemporaryIdAllocator::new_id() you should make sure it's documented and guarded
    let permanent_id: Id<RecordWithCustomId> = Id::Permanent{
        value: Char24IdAllocator::new_id()
    };
    println!("Permanent id: {:?}", permanent_id);

    let cloned_permanent_id: Id<RecordWithCustomId> = permanent_id.clone();

    assert_eq!(permanent_id, cloned_permanent_id);
}