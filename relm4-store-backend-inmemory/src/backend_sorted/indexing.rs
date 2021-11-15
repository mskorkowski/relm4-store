// use std::cmp::Ord;
// use std::cmp::PartialOrd;
// use std::cmp::PartialEq;
// use std::cmp::Eq;
// use std::collections::HashMap;
// use std::collections::BTreeSet;

// use std::hash::Hash;

// use std::ops::Range;


// #[derive(Clone)]
// enum Keys {
//     IncreaseOrderKey(i32),
//     DecreaseOrderKey(i32),
// }

// impl PartialEq for Keys {
//     fn eq(&self, other: &Self) -> bool {
//         match (self, other) {
//             (Keys::IncreaseOrderKey(l), Keys::IncreaseOrderKey(r)) => {
//                 *l == *r
//             },
//             (Keys::DecreaseOrderKey(l), Keys::DecreaseOrderKey(r)) => {
//                 *l == *r
//             },
//             _ => false
//         }
//     }
// }

// impl Eq for Keys {}

// impl PartialOrd for Keys {
//     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//         match (self, other) {
//             (Keys::IncreaseOrderKey(l), Keys::IncreaseOrderKey(r)) => {
//                 l.partial_cmp(r)
//             },
//             (Keys::DecreaseOrderKey(l), Keys::DecreaseOrderKey(r)) => {
//                 l.partial_cmp(r).map(|o| o.reverse())
//             },
//             _ => None
//         }
//     }    
// }

// impl Ord for Keys {
//     fn cmp(&self, other: &Self) -> std::cmp::Ordering {
//         if let Some(ordering) = self.partial_cmp(other) {
//             ordering
//         }
//         else {
//             panic!("This is hacky Ord implementation. It doesn't work for diverging keys");
//         }
//     }
// }

// struct OuterKey(Keys);


// impl PartialEq for OuterKey {
//     fn eq(&self, other: &Self) -> bool {
//         match (&self.0, &other.0) {
//             (Keys::IncreaseOrderKey(_), Keys::IncreaseOrderKey(_)) => {
//                 true
//             },
//             (Keys::DecreaseOrderKey(_), Keys::DecreaseOrderKey(_)) => {
//                 true
//             },
//             _ => false
//         }
//     }
// }

// impl Eq for OuterKey {}

// impl Hash for OuterKey {
//     fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
//         match self.0 {
//             Keys::IncreaseOrderKey(_) => state.write_u8(1),
//             Keys::DecreaseOrderKey(_) => state.write_u8(2),
//         }
//     }
// }

// struct Indexes<OuterKey, Keys> {
//     indexes: HashMap<OuterKey, BTreeSet<Keys>>
// }

// trait OuterKey {
//     type Keys;
//     fn from(k: Self::Keys) -> Self;
// }

// trait InnerKey: Sized {
//     type Record: record::Record;

//     fn get_id(&self) -> record::Id<Self::Record>;
//     fn build(record: &Self::Record) -> Vec<Self>;
// }

// struct Range<'a, T: 'a> {
//     iter: std::collections::btree_set::Range<'a, T>
// }

// impl<OuterKey, Keys> Indexes<OuterKey, Keys> 
// where
//     OuterKey: Hash + self::OuterKey<Keys=Keys> + Eq,
//     Keys: Clone + Ord,
// {
//     fn range<R>(&self, k: Keys, r: R) -> Range<'_, Keys> {
//         if let Some(set) = self.indexes.get(&OuterKey::from(k.clone())) {
//             Range{
//                 iter: set.range(&r)
//             }
//         }
//         else {
//              //no btree = no value
//         }
//     }

//     fn insert(&mut self, k: Keys, v: i32) {
//         if let Some(mut btree) = self.indexes.get_mut(&OuterKey(k.clone())) {
//             btree.insert(k, v);
//         }
//         else {
//             let mut btree = BTreeMap::new();
//             btree.insert(k.clone(), v);
//             self.indexes.insert(OuterKey(k), btree);
//         }
//     }
// }