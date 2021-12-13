use super::*;


impl<Value, Comparator> TreeImpl<Value, Comparator> {

    // fn add_internal(&mut self, e: Value, index: Option<usize>) -> &Value {
    //     let mut n: *mut Node<Value, Comparator>;
    //     let mut np: *mut *mut Node<Value, Comparator>;
    //     let mut left: *mut Node<Value, Comparator>;
    //     let mut right: *mut Node<Value, Comparator>;
    //     let mut orig_e: *const Value = &e;
    //     let c: usize;
    //     let lcount: usize;
    //     let rcount: usize;

    //     let node_layout = Layout::new::<Node<Value, Comparator>>();

        
    //     if self.count == 0 {
    //         assert!(node_layout.size() <= isize::MAX as usize, "Unable to allocate root node. Node size is too big");
    //         // root is empty, no data in the tree yet!
    //         let root_ptr = unsafe {
    //             alloc(node_layout) as *mut Node<Value, Comparator>
    //         };

    //         let mut root = unsafe { NonNull::new_unchecked(root_ptr) };
            
    //         let result = unsafe{
    //             let r = root.as_mut();
    //             r.leaf = false;
    //             r.elems.push_front(e);
    //             r.elems.get(0).unwrap()
    //         };

    //         self.root = root;
    //         self.count += 1;

    //         return result
    //     }
    //     else {
    //         unsafe{
    //             self.root.as_mut().insert_non_full(e, index)
    //         }
    //     }
    // }

}

impl<Value, Comparator> Node<Value, Comparator> {

    // Finds index in the vec of kids
    // 
    // If this is leaf node it will be an insertion index
    // If this is not a leaf node then it will be index of the kid to perform insert
    // fn find_index_in_node(&self, e: &Value) -> usize {
    //     self.elems.
    // }

    // A utility function to insert a new key in this node
    // The assumption is, the node must be non-full when this
    // function is called
    // fn insert_non_full(&mut self, e: Value, index: Option<usize>) -> &Value {
        
    // }

    /* void BTreeNode::insertNonFull(int k)
    {
        # Initialize index as index of rightmost element
        int i = n-1;
    
        # If this is a leaf node
        if (leaf == true)
        {
            # The following loop does two things
            #  a) Finds the location of new key to be inserted
            #  b) Moves all greater keys to one place ahead
            while (i >= 0 && keys[i] > k)
            {
                keys[i+1] = keys[i];
                i--;
            }
    
            # Insert the new key at found location
            keys[i+1] = k;
            n = n+1;
        }
        else # If this node is not leaf
        {
            # Find the child which is going to have the new key
            while (i >= 0 && keys[i] > k)
                i--;
    
            # See if the found child is full
            if (C[i+1]->n == 2*t-1)
            {
                # If the child is full, then split it
                splitChild(i+1, C[i+1]);
    
                # After split, the middle key of C[i] goes up and
                # C[i] is split into two.  See which of the two
                # is going to have the new key
                if (keys[i+1] < k)
                    i++;
            }
            C[i+1]->insertNonFull(k);
        }
    }*/

}
