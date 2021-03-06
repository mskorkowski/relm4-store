use reexport::log;

use store::Backend;
use store::Replies;
use store::StoreViewMsg;

use std::cmp::min;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::fmt::Debug;

use record::Id;
use record::Record;
use store::Position;
use store::StoreMsg;
use store::math::Range;

/// Configuration trait for the InMemoryBackend
pub trait InMemoryBackendConfiguration {
    /// Type of data in the in memory store
    type Record: 'static + Record + Debug + Clone;

    /// Returns initial dataset for the store
    fn initial_data() -> Vec<Self::Record>;
}

/// In memory implementation of the data store
#[derive(Debug)]
pub struct InMemoryBackend<Configuration> 
where 
    Configuration: InMemoryBackendConfiguration,
{
    /// Order of profiles
    order: VecDeque<Id<Configuration::Record>>,

    /// profile storage
    data: HashMap<Id<Configuration::Record>, Configuration::Record>,
}

impl<Configuration> InMemoryBackend<Configuration> 
where 
    Configuration: InMemoryBackendConfiguration + 'static,
{
    /// Creates new instance of the InMemoryBackend
    pub fn new() -> Self {
        let mut backend = InMemoryBackend {
            order: VecDeque::new(),
            data: HashMap::new(),
        };

        for record in Configuration::initial_data() {
            backend.add(record);
        }

        backend
    }

    fn add(&mut self, record: Configuration::Record) -> Position {
        let id = record.get_id();
        {
            self.data.insert(id, record);
            self.order.push_back(id);

            Position(self.order.len() -1)
        }
    }
}

impl<Configuration> Backend for InMemoryBackend<Configuration>
where 
    Configuration: 'static + InMemoryBackendConfiguration,
{
    type Record = Configuration::Record;

    fn len(&self) -> usize {
        self.data.len()
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn get_range(&self, range: &Range) -> Vec<Self::Record> {
        let count = self.len();

        let start = min(*range.start(), count);
        let length = min(*range.end(), count) - start;

        let iter = self.order.range(start..(start+length));

        let mut result: Vec<Self::Record> = Vec::new();

        for id in iter {
            let record = {
                self.data.get(id).unwrap().clone()
            };

            result.push(record);
        }

        result
    }

    fn get(&self, id: &Id<Configuration::Record>) -> Option<Configuration::Record> {
        let data = &self.data;
        data.get(id).cloned()
    }

    fn inbox(&mut self, msg: StoreMsg<Configuration::Record>) -> Replies<Configuration::Record> {
        log::info!("Received message: {:?}", &msg);

        let mut replies = vec!();

        match msg {
            StoreMsg::Commit(record) => {
                let id = record.get_id();
                {
                    if id.is_new() {
                        let position = self.add(record);
                        replies.push(StoreViewMsg::NewAt(position));
                    }
                    else {
                        self.data.insert(id, record);
                        replies.push(StoreViewMsg::Update(id));
                    }
                }

            },
            StoreMsg::Delete(id) => {
                if self.data.contains_key(&id) {
                    self.data.remove(&id);
                    
                    let mut order_idx = None;

                    for (idx, oid) in self.order.iter().enumerate() {
                        if *oid == id {
                            order_idx = Some(idx);
                        }
                    }

                    if let Some(idx) = order_idx {
                        self.order.remove(idx);
                        replies.push(StoreViewMsg::Remove(Position(idx)));
                    }

                }
            },
            StoreMsg::Reload => {
                //it's in memory store so nothing to do...
            }, 
        };

        Replies{
            replies
        }
    }
}

impl<Configuration> Default for InMemoryBackend<Configuration> 
where 
    Configuration: InMemoryBackendConfiguration + 'static,
{
    fn default() -> Self {
        Self::new()
    }
}