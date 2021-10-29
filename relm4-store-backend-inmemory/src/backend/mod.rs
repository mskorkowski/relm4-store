mod component;

use std::cmp::min;
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::fmt::Debug;

use model::Id;
use model::Identifiable;
use model::Model;
use store::DataStore;
use store::DataStoreBase;
use store::DataStoreListenable;
use store::Handler;
use store::IdentifiableStore;
use store::RecordWithLocation;
use store::Position;
use store::StoreId;
use store::StoreMsg;
use store::math::Range;

pub trait InMemoryBackendBuilder {
    type DataModel: 'static + Model + Debug + Clone;

    fn initial_data() -> Vec<Self::DataModel>;
}

pub struct InMemoryBackend<Builder> 
where Builder: InMemoryBackendBuilder,
{
    id: StoreId<Self>,

    /// Order of profiles
    order: RefCell<VecDeque<Id<Builder::DataModel>>>,

    /// profile storage
    data: RefCell<HashMap<Id<Builder::DataModel>, Builder::DataModel>>,

    handlers: RefCell<HashMap<StoreId<Self>, Box<dyn Handler<Self>>>>,
}

impl<Builder> InMemoryBackend<Builder> 
where Builder: InMemoryBackendBuilder
{
    pub fn new(initial_data: Vec<Builder::DataModel>) -> Self {
        // println!("{}:{}", file!(), line!());
        let backend = InMemoryBackend{
            id: StoreId::new(),
            order: RefCell::new(VecDeque::new()),
            data: RefCell::new(HashMap::new()),
            handlers: RefCell::new(HashMap::new()),
        };

        for record in initial_data {
            backend.inbox(StoreMsg::New(record));
        }

        //we don't have any views so we don't need to notify anybody yet

        // println!("\tbackend created");
        backend
    }

    fn fire_handlers(&self, message: StoreMsg<Builder::DataModel>) {
        let handlers = self.handlers.borrow();

        if handlers.is_empty() {
            return;
        }

        // tracks store view id's for removal
        //
        // If handler return `true` from `handle` method it should be removed
        //
        // we borrow the self.handlers in the for loop and to do
        // removal we need to borrow again since, unlisten is
        // internally mutable which would cause UB, since we would 
        // iterate over collection which changes itself
        let mut ids_for_remove: Vec<StoreId<Self>> = Vec::new();

        for (key, handler) in handlers.iter() {
            // println!("\tNotifying store: {:?}", key);

            let remove = handler.handle(message.clone());

            if remove {
                ids_for_remove.push(*key);
            }
        }

        // cleanup all handler which decided to remove itself
        for id in ids_for_remove {
            self.unlisten(id);
        }
    }

    fn add(&self, record: Builder::DataModel) -> Position {
        let id = record.get_id();
        {
            self.data.borrow_mut().insert(id, record.clone());
            let mut order = self.order.borrow_mut();
            order.push_back(id);

            Position(order.len() -1)
        }
    }
}

impl<Builder> Identifiable for InMemoryBackend<Builder>
where Builder: InMemoryBackendBuilder
{
    type Id=StoreId<Self>;

    fn get_id(&self) -> Self::Id {
        // println!("{}:{}", file!(), line!());
        self.id
    }
}

impl<Builder> IdentifiableStore for InMemoryBackend<Builder> 
where Builder: InMemoryBackendBuilder,
{}

impl<Builder> DataStoreBase for InMemoryBackend<Builder>
where Builder: InMemoryBackendBuilder,
{
    type Model = Builder::DataModel;

    fn inbox(&self, msg: StoreMsg<Builder::DataModel>) {
        // println!("{}:{}", file!(), line!());

        match msg {
            StoreMsg::New(record) => {
                // println!("{}:{}: New record", file!(), line!());
                let position = self.add(record);

                self.fire_handlers(
                    StoreMsg::NewAt(position)
                );
            },
            StoreMsg::Commit(record) => {
                // println!("{}:{}: Commit", file!(), line!());

                let id = record.get_id();
                {
                    let mut data = self.data.borrow_mut();
                    
                    // let old_record = data.get(&id);
                    // println!("\t{:?} -> {:?}", old_record, record);

                    data.insert(id, record);
                }

                self.fire_handlers(StoreMsg::Update(id))
            },
            StoreMsg::Reload => {
                //it's in memory store so nothing to do...
                // println!("{}:{}: Reload", file!(), line!());
            }, 
            _ => {
                // println!("{}:{}: Default", file!(), line!());
            }
        }

    }

    fn len(&self) -> usize {
        // println!("{}:{}", file!(), line!());
        self.data.borrow().len()
    }

    fn is_empty(&self) -> bool {
        // println!("{}:{}", file!(), line!());
        self.len() == 0
    }

    fn get_range(&self, range: &Range) -> Vec<RecordWithLocation<Self::Model>> {
        // println!("{}:{}", file!(), line!());
        let count = self.len();

        let start = min(range.start(), count);
        let length = min(range.end(), count) - start;

        let order = self.order.borrow();
        let iter = order.range(start..(start+length));

        let mut result: Vec<RecordWithLocation<Self::Model>> = Vec::new();

        for (i, id) in iter.enumerate() {
            let record = {
                self.data.borrow().get(id).unwrap().clone()
            };

            let v = RecordWithLocation::new(Position(start+i), record);

            result.push(v);
        }

        result
    }

    fn get(&self, id: &Id<Builder::DataModel>) -> Option<(Position, Builder::DataModel)> {
        // println!("{}:{}", file!(), line!());
        let order = self.order.borrow();
        let position = order.iter().enumerate().find(|(_pos, e)| **e == *id);
        let profiles = self.data.borrow();
        let record = profiles.get(id);

        position.map(move |pos| record.map(move |prof| (Position(pos.0), prof.clone()) )).flatten()
    }
}

impl<Builder> DataStoreListenable for InMemoryBackend<Builder> 
where Builder: InMemoryBackendBuilder
{
    fn listen<'b>(&self, handler_ref: StoreId<Self>, handler: Box<dyn Handler<Self>>) {
        // println!("{}:{}", file!(), line!());
        self.handlers.borrow_mut().insert(handler_ref, handler);
    }

    fn unlisten(&self, handler_ref: StoreId<Self>) {
        // println!("{}:{}", file!(), line!());
        self.handlers.borrow_mut().remove(&handler_ref);
    }
}

impl<Builder> DataStore for InMemoryBackend<Builder>
where Builder: InMemoryBackendBuilder
{}