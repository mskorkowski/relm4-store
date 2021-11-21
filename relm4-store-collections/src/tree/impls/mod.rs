//! Craziness is here
//! 
//! Here we have an implementation of our tree. It's unsafe as c can be
//! 
//! This implementation is based of `tree234` by Simon Tatham. Original can
//! be found on [Simon Tatham's home page](https://www.chiark.greenend.org.uk/~sgtatham/algorithms/tree234.c)
//! 
//! Some ideas was taken from "Introduction to algorithms, 2nd edition" by Cormen, Leiserson, Rivest, Stein
//! 
//! Internally whatever is below this module is part of the implementation of this tree
//! I've split it up so I can understand it in parts

mod drop;
mod insert;
mod len;
mod new;

// use std::alloc::alloc;
// use std::alloc::Layout;
use std::collections::VecDeque;
use std::marker::PhantomData;
use std::ptr::NonNull;

use crate::TreeConfiguration;

pub struct TreeImpl<Value, Comparator> {
    root: NonNull<Node<Value, Comparator>>,
    configuration: TreeConfiguration,
    count: usize,
    _comparator: PhantomData<*mut Comparator>, //invariance in comparator
    _value: PhantomData<Value>, // forces covariance over Value
}

struct Node<Value, Comparator> {
    parent: *const Node<Value, Comparator>,

    leaf: bool,

    kids: VecDeque<CountedNode<Value, Comparator>>,

    elems: VecDeque<Value>,
    _comparator: PhantomData<*mut Comparator>, //invariance in comparator
    _value: PhantomData<Value>, // forces covariance over Value
}

struct CountedNode<Value, Comparator> {
    node: NonNull<Node<Value, Comparator>>,
    count: usize,
    _comparator: PhantomData<*mut Comparator>, //invariance in comparator
    _value: PhantomData<Value>, // forces covariance over Value
}






