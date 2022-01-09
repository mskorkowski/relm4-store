use reexport::log;
use store::Backend;
use store::OrderedBackend;
use store::Replies;
use store::Sorter;
use store::StoreViewMsg;

use std::cmp::min;
use std::collections::HashMap;
use std::fmt::Debug;
use record::Id;
use record::Record;
use record::TemporaryIdAllocator;
use store::Position;
use store::StoreMsg;
use store::math::Range;

/// Configuration trait for the SortedInMemoryBackend
pub trait SortedInMemoryBackendConfiguration {
    /// Type of data in the in memory store
    type Record: 'static + Record + Debug + Clone;
    /// Type of the ordering
    type OrderBy: 'static + Sorter<Self::Record> + Copy;

    /// Returns initial dataset for the store
    fn initial_data() -> Vec<Self::Record>;

    /// Returns ordering at the backend creation time
    fn initial_order() -> Self::OrderBy;
}

/// In memory implementation of the data store
#[derive(Debug)]
pub struct SortedInMemoryBackend<Config> 
where 
    Config: SortedInMemoryBackendConfiguration,
{
    /// Order of profiles
    order: Vec<Id<Config::Record>>,

    /// profile storage
    data: HashMap<Id<Config::Record>, Config::Record>,

    ordering: Config::OrderBy
}

impl<Config> SortedInMemoryBackend<Config> 
where 
    Config: SortedInMemoryBackendConfiguration + 'static,
{
    /// Creates new instance of the InMemoryBackend
    pub fn new() -> Self {
        let mut backend = SortedInMemoryBackend{
            order: Vec::new(),
            data: HashMap::new(),
            ordering: Config::initial_order(),
        };
        
        let initial_data = Config::initial_data();
        for record in initial_data {
            backend.add(record);
        }

        backend
    }

    fn add(&mut self, mut record: Config::Record) -> Replies<Config::Record> {
        let mut replies = vec!();
        let id = record.get_id();
        if id.is_new() {
            record.set_permanent_id(<<Config::Record as Record>::Allocator as TemporaryIdAllocator>::new_id()).expect(&format!("Unable to set the permanent id for record `{:#?}`", record));
            let position = self.insert(record);
            replies.push(StoreViewMsg::NewAt(position));
        }
        else if !self.data.contains_key(&id) {
            let position = self.insert(record);
            replies.push(StoreViewMsg::NewAt(position));
        }
        else {
            replies.push(self.update(record));
        }

        Replies{
            replies
        }
    }

    /// Calls to this method are allowed only if you add nonexisting record to the store
    /// 
    /// Running it for existing record is undefined and might happily destroy your data. 
    fn insert(&mut self, record: Config::Record) -> Position {
        let id = record.get_id();
        self.data.insert(id, record.clone());

        let position = {
            let ordering = self.ordering;
            let r = record.clone();
            self.order.binary_search_by(|other_id| {
                let other = self.data.get(other_id).unwrap();
                ordering.cmp(&r, other).reverse()
            })
        };

        let pos = match position {
            // if two elements are equal it doesn't matter which one is first
            Ok(p) => {
                self.order.insert(p, id);
                Position(p)
            },
            Err(p) => {
                self.order.insert(p, id);
                Position(p)
            }
        };

        pos
    }

    /// Calls to this method are allowed only if record is already in the store
    /// 
    /// Running if you run it for nonexisting record this method is undefined. It might end up with panic or
    /// killing your cat, or flooding, or erupting volcanos, or whatever other disaster you might think of. 
    fn update(&mut self, record: Config::Record) -> StoreViewMsg<Config::Record> {
        let id = record.get_id();
        // record is already in store => it's safe to unwrap
        let old_record = self.data.insert(id, record.clone()).unwrap();

        let old_position: usize = {
            let ordering = self.ordering;
            let position = self.order.binary_search_by(|other_id| {
                let other = self.data.get(other_id).unwrap();
                ordering.cmp(&old_record, other).reverse()
            });

            match position {
                Ok(p) => p,
                Err(_) => panic!("Record doesn't exist in order while it's in the data! Performing seppuku!")
            }
        };


        let position = {
            let ordering = self.ordering;
            let r = record.clone();
            self.order.binary_search_by(|other_id| {
                let other = self.data.get(other_id).unwrap();
                ordering.cmp(&r, other).reverse()
            })
        };

        let (to, mut from, p) = match position {
            Ok(p) => {
                // if two elements are equal it doesn't matter which one is first
                if p == old_position {
                    (p, old_position, StoreViewMsg::Update(id))
                }
                else if p < old_position {
                    (old_position, p, StoreViewMsg::Move{from: Position(old_position), to: Position(p)} )
                }
                else {
                    (p, old_position, StoreViewMsg::Move{from: Position(old_position), to: Position(p)} )
                }
            },
            Err(p) => {
                if p == old_position {
                    (p, old_position, StoreViewMsg::Update(id))
                }
                else if p < old_position {
                    (old_position, p, StoreViewMsg::Move{from: Position(old_position), to: Position(p)} )
                }
                else {
                    (p, old_position, StoreViewMsg::Move{from: Position(old_position), to: Position(p)} )
                }
            }
        };

        if from != to {
            // perform minimum reorder of view for the case when records where updated
            while to >= from {
                self.order[from] = self.order[from-1];
                from -= 1;
            }
            self.order[to] = id;
        }

        p
    }
}

impl<Configuration> Backend for SortedInMemoryBackend<Configuration>
where 
    Configuration: 'static + SortedInMemoryBackendConfiguration,
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

        let mut result: Vec<Self::Record> = Vec::new();

        for idx in start..(start+length) {
            let id = &self.order[idx];
            let record = self.data.get(id).unwrap().clone();
            result.push(record);
        }

        result
    }

    fn get(&self, id: &Id<Configuration::Record>) -> Option<Configuration::Record> {
        self.data.get(id)
            .map(|r| r.clone())
    }

    fn inbox(&mut self, msg: StoreMsg<Configuration::Record>) -> Replies<Configuration::Record> {
        log::info!("Received message: {:?}", &msg);
        match msg {
            StoreMsg::Commit(record) => {
                self.add(record)
            },
            StoreMsg::Reload => {
                //it's in memory store so nothing to do...
                Replies{
                    replies: vec!()
                }
            }, 
            StoreMsg::Delete(id) => {
                let mut replies = vec![];
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

                Replies{
                    replies,
                }
            },
        }
    }
}

impl<Config> OrderedBackend<Config::OrderBy> for  SortedInMemoryBackend<Config> 
where 
    Config: SortedInMemoryBackendConfiguration + 'static,
{
    fn set_order(&mut self, ordering: Config::OrderBy) -> Replies<Config::Record> {
        let mut ordered_data_before = vec![];
        
        for idx in self.order.iter() {
            ordered_data_before.push(self.data[idx].clone())
        }
        
        self.ordering = ordering;

        self.order.sort_by(|lhs, rhs| {
            let l = &self.data[lhs];
            let r = &self.data[rhs];
            ordering.cmp(l, r)
        });

        let mut ordered_data_after = vec![];
        for idx in self.order.iter() {
            ordered_data_after.push(self.data[idx].clone())
        }

        Replies{
            replies: vec!(StoreViewMsg::Reload)
        }
    }
}