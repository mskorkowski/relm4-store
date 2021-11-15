# Ordering in Rust (or rather lack of it)

**Disclaimer** In here I'm not going to discuss sorting algorithms. For sake of this article if I have an algorithm which takes the set of data and returns the copy of it (modifies given set) in such a way that accessing consecutive elements of the given set presents them in order that's fine. Even infamous random sort will work (in theory).

**Disclaimer 2** This rambling is just me venting my frustration about some issues I've hit while implementing this library. It might be interesting to somebody since I'm discussing some indexing internals of the data-store. This article is relevant to the `InMemoryBackend` implementation. Other stores
might not provide indexing or use some other techniques since implementing store is really dependant on the data storage medium.

Rust as language has a lots of nice features but it lacks a lot in department of sorting. How you ask? Rust allows you to define only one kind of ordering for the type. For sake of discussion let's call in `natural order`. For simple types it's mostly right. But for types which are more complex you can define multiple kinds of orders which are [`total orders`](https://en.wikipedia.org/wiki/Total_order) in mathematical sense.

Let's talk in examples to make more sense about my complains. This is simple example of sorting integers. You can found it in [rust-lang-nursery](https://rust-lang-nursery.github.io/rust-cookbook/algorithms/sorting.html#sort-a-vector-of-structs) at the time of writing this.

```rust
fn main() {
    let mut vec = vec![1, 5, 10, 2, 15];
    
    vec.sort();

    assert_eq!(vec, vec![1, 2, 5, 10, 15]);
}
```

That was easy. Let's try to sort this numbers in reverse then.

```rust
fn main() {
    let mut vec = vec![1, 5, 10, 2, 15];
    
    vec.sort_by(|a, b| a.partial_cmp(b).unwrap().reverse());

    assert_eq!(vec, vec![15, 10, 5, 2, 1]);
}
```

Since I can order the numbers let's talk about taking smallest element form a vec. In both examples it's simply takeing reference to the 0th element of the vector.

Let's try to do the same with [`BTreeMap`](./https://doc.rust-lang.org/std/collections/struct.BTreeMap.html).

```rust
use std::collections::BTreeMap;

fn main() {
    let mut collection = BTreeMap::new();
    
    collection.insert(1, 1);
    collection.insert(5, 5);
    collection.insert(10, 10);
    collection.insert(2, 2);
    collection.insert(15, 15);
    
    let vec: Vec<i32> = collection.values().map(|i| *i).collect();
    assert_eq!(vec, vec![1, 2, 5, 10, 15]);
}
```

Now since type of key for BTreeMap must implement `Ord` it means that only whatever we call "natural order" can be used to keep values in the BTreeMap. If you would like to keep two indexes to values inside a BTreeMap for the same Key you are out of luck. One type of Key one ordering.

How to go around this bullshit requirement?

In here I'll ramble how my ideal world would look like. Fill free to skip it to the `Way 3` section. `Way 1` and `Way 2` is not going to introduce anything useful.

## Way 1: Follow HashMap/HashSet

`HashMap` and `HashSet` allows to provide `Hasher`. Making optional constructor for `BTreeMap` to provide an order implementation over key would be perfect.

## Way 2: Fix the standard library

I'm not smart enough and definitely not patient enough to make it happen.

Rough sketch of the fix would go like this. Make `Ord` take a type argument. Make `PartialOrd`,`PartialEq`,etc.. take a two arguments instead of one.

On `PartialOrd` example

```rust
pub trait PartialOrd<Rhs: ?Sized = Self, Lhs: ?Sized = Self>: PartialEq<Rhs, Lhs> {
    ...
}
```

And for `Ord`

```rust
pub trait Ord<Type=Self>: Eq + PartialOrd<Type, Type> {
    ...
}
```

`BTreeMap` could go as

```rust

pub struct BTreeMap<K, V, Ordering=K> {
    root: Option<Root<K, V>>,
    length: usize,
}

```

This way you could create a custom structure with different type of ordering.

```rust
pub struct CustomNumberOrder {}

impl Ord<i32> for CustomNumberOrder {} //and you must implement all of the required traits properly
```

and you could use the `BTreeMap` as such:

```rust
    let myMap: BTreeMap<i32, V, CustomNumberOrder> = BTreeMap::new();
```

Now the definition of `<`, `<=`, etc... as things aliased to the `Ord` is less fun but can be retained in a form operator `<` can be used for types for which `Ord<Self>` is implemented.

This is a sketch of humongous amount of work for which I'm only see a complains (and I'm too lazy). So let's go back to the real world.

## Way 3: Define your own key type

Since you are here, that means you are like me, too lazy to mess with standard library.

Ok so how do I solve this broken issue? First we need to define one structures for each ordering type. Each of them will keep values in one kind of order.
In our simple number example it could go like:

```rust

struct IncreaseOrderKey(i32);
struct DecreaseOrderKey(i32);

impl Ord for IncreaseOrderKey(i32) {}
impl Ord for DecreaseOrderKey(i32) {}

```

Now we can create two maps one `BTreeMap<IncreaseOrderKey, i32>` and the other `BTreeMap<DecreaseOrderKey, i32>` and it works. Somewhat.

The biggest issue with this, is that both maps have a different signature. So how can I get the thing working?

### Enums to the rescue

```rust
use std::cmp::Ord;
use std::cmp::PartialOrd;
use std::cmp::PartialEq;
use std::cmp::Eq;

enum Keys {
    IncreaseOrderKey(i32),
    DecreaseOrderKey(i32),
}

impl PartialEq for Keys {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Keys::IncreaseOrderKey(l), Keys::IncreaseOrderKey(r)) => {
                *l == *r
            },
            (Keys::DecreaseOrderKey(l), Keys::DecreaseOrderKey(r)) => {
                *l == *r
            },
            _ => false
        }
    }
}

impl Eq for Keys {}

impl PartialOrd for Keys {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Keys::IncreaseOrderKey(l), Keys::IncreaseOrderKey(r)) => {
                l.partial_cmp(r)
            },
            (Keys::DecreaseOrderKey(l), Keys::DecreaseOrderKey(r)) => {
                l.partial_cmp(r).map(|o| o.reverse())
            },
            _ => None
        }
    }    
}

impl Ord for Keys {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if let Some(ordering) = self.partial_cmp(other) {
            ordering
        }
        else {
            panic!("This is hacky Ord implementation. It doesn't work for diverging keys");
        }
    }
}

```

Now we can define two maps with same signature but different kind of ordering. Only issue is we must be careful about which map is which. This can be found as the `panic!` line in the `Ord` implementation. We've broke the `Ord` contract since it's `Ord` just for a specific values of `Keys` and not a whole `Keys` as such. Famous last words "I'll be careful. I promise!"

I don't like to be on the receiving end of being careful. So let's solve this issue with another layer of indirection.

```rust

struct OuterKey(Keys);


impl PartialEq for OuterKey {
    fn eq(&self, other: &Self) -> bool {
        match (&self.0, &other.0) {
            (Keys::IncreaseOrderKey(_), Keys::IncreaseOrderKey(_)) => {
                true
            },
            (Keys::DecreaseOrderKey(_), Keys::DecreaseOrderKey(_)) => {
                true
            },
            _ => false
        }
    }
}

impl Eq for OuterKey {}

impl Hash for OuterKey {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self.0 {
            Keys::IncreaseOrderKey(_) => state.write_u8(1),
            Keys::DecreaseOrderKey(_) => state.write_u8(2),
        }
    }
}

struct Indexes {
    indexes: HashMap<OuterKey, BTreeMap<Keys, i32>>
}

impl Indexes {
    fn get(&self, k: Keys) -> Option<&i32> {
        if let Some(btree) = self.indexes.get(&OuterKey(k.clone())) {
            btree.get(&k)
        }
        else {
            None //no btree = no value
        }
    }

    fn insert(&mut self, k: Keys, v: i32) {
        if let Some(mut btree) = self.indexes.get_mut(&OuterKey(k.clone())) {
            btree.insert(k, v);
        }
        else {
            let mut btree = BTreeMap::new();
            btree.insert(k.clone(), v);
            self.indexes.insert(OuterKey(k), btree);
        }
    }
}

```

Now if somebody will provide `OuterKey` and vector of allowed `Keys` you can even go beyond that. Either way implementing this is pitiful and painful. The fact of abusing the spec and reinventing a wheel is sad. Seriously... I'm sad...

## Summary

To implement it in decent way you need to be extra careful about preserving properties of ordering. If you half ass that you are going into world of pain much bigger then you would expect. You are going to loose the data. Especially that one of the first thing we did was breaking the rules so we've `panicked` while implementing `Ord` for `Keys`.
