use node::Node;
use memrange::Range;
use node::{insert,delete,search,min_pair, max_pair, height};
use iterators::RangePairIter;

#[derive(Debug)]
pub struct IntervalTree<D> {
    pub root: Option<Box<Node<D>>>
}

impl <D> IntervalTree<D>{


/// This function will construct a new empty IntervalTree.
/// # Examples
/// ```
/// extern crate theban_interval_tree;
/// let mut t=theban_interval_tree::IntervalTree::<i32>::new();
/// ```
    pub fn new() -> IntervalTree<D>{
        IntervalTree{root: None}
    }

/// This function will insert the key,value pair into the tree, overwriting the old data if the key is allready
/// part of the tree.
/// # Examples
/// ```
/// extern crate memrange;
/// extern crate theban_interval_tree;
///
/// let mut t=theban_interval_tree::IntervalTree::<i32>::new();
/// t.insert(memrange::Range::new(2,2),25);
/// assert_eq!(t.get(memrange::Range::new(2,2)), Some(&25));
/// t.insert(memrange::Range::new(2,2),30);
/// assert_eq!(t.get(memrange::Range::new(2,2)), Some(&30));
/// ```
    pub fn insert(&mut self, key: Range, data: D) {
        match self.root.take() {
            Some(box_to_node) => self.root = Some(insert::<D>(key, data, box_to_node)),
            None => self.root = Some(Box::new(Node::new(key,data))),
        }
    }

/// This function will remove the key,value pair from the tree, doing nothing if the key is not
/// part of the tree.
/// # Examples
/// ```
/// extern crate memrange;
/// extern crate theban_interval_tree;
///
/// let mut t=theban_interval_tree::IntervalTree::<i32>::new();
/// t.insert(memrange::Range::new(2,2),25);
/// t.delete(memrange::Range::new(2,2));
/// assert!(t.empty());
/// // deleting nonexistant keys doesn't do anything
/// t.delete(memrange::Range::new(3,3));
/// assert!(t.empty());
/// ```
    pub fn delete(&mut self, key: Range){
        match self.root.take() {
            Some(box_to_node) => self.root = delete(key,box_to_node),
            None => return
        }
    }

/// This function will return the Some(data) stored under the given key or None if the key is not
/// known.
/// # Examples
/// ```
/// extern crate memrange;
/// extern crate theban_interval_tree;
///
/// let mut t=theban_interval_tree::IntervalTree::<i32>::new();
/// t.insert(memrange::Range::new(2,2),25);
/// assert_eq!(t.get(memrange::Range::new(2,2)), Some(&25));
/// assert_eq!(t.get(memrange::Range::new(3,3)), None);
///
/// ```
    pub fn get(&self, key: Range) -> Option<&D>{
        match self.root {
            Some(ref box_to_node) =>search(&key, box_to_node),
            None => None
        }
    }

/// This function will return the data stored under the given key or the default if the key is not
/// known.
/// # Examples
/// ```
/// extern crate memrange;
/// extern crate theban_interval_tree;
///
/// let mut t=theban_interval_tree::IntervalTree::<i32>::new();
/// t.insert(memrange::Range::new(2,2),25);
/// assert_eq!(t.get_or(memrange::Range::new(2,2),&2000), &25);
/// assert_eq!(t.get_or(memrange::Range::new(3,3),&2000), &2000);
///
/// ```
    pub fn get_or<'a>(&'a self, key: Range, default: &'a D) -> &D{
        self.get(key).map_or(default, |data| data)
    }

/// This function will return true if the tree contains the given key, false otherwise
/// # Examples
/// ```
/// extern crate memrange;
/// extern crate theban_interval_tree;
///
/// let mut t=theban_interval_tree::IntervalTree::<i32>::new();
/// t.insert(memrange::Range::new(2,2),25);
/// assert!(!t.contains(memrange::Range::new(3,3)));
/// assert!(t.contains(memrange::Range::new(2,2)));
///
/// ```
    pub fn contains(&self, key: Range) -> bool {
        self.get(key).is_some()
    }

/// This function will return true if the tree is empty, false otherwise.
/// # Examples
/// ```
/// extern crate memrange;
/// extern crate theban_interval_tree;
///
/// let mut t=theban_interval_tree::IntervalTree::<i32>::new();
/// assert!(t.empty());
/// t.insert(memrange::Range::new(2,2),25);
/// assert!(!t.empty());
///
/// ```
    pub fn empty(&self) -> bool { self.root.is_none() }

/// This function will return the key/value pair with the smallest key in the tree, or None if the
/// tree is empty.
/// # Examples
/// ```
/// extern crate memrange;
/// extern crate theban_interval_tree;
///
/// let mut t=theban_interval_tree::IntervalTree::<u64>::new();
/// t.insert(memrange::Range::new(2,2),25);
/// t.insert(memrange::Range::new(3,3),50);
/// assert_eq!(t.min().unwrap().0, &memrange::Range::new(2,2));
/// assert_eq!(t.min().unwrap().1, &25);
///
/// ```
    pub fn min<'a>(&'a self) -> Option<(&'a Range,&'a D)> {
        match self.root {
            Some(ref root) => Some(min_pair(root)),
            None => None
        }
    }

/// This function will return the key/value pair with the biggest key in the tree, or None if the
/// tree is empty.
/// # Examples
/// ```
/// extern crate memrange;
/// extern crate theban_interval_tree;
///
/// let mut t=theban_interval_tree::IntervalTree::<i32>::new();
/// t.insert(memrange::Range::new(2,2),25);
/// t.insert(memrange::Range::new(3,3),50);
/// assert_eq!(t.max().unwrap().0, &memrange::Range::new(3,3));
/// assert_eq!(t.max().unwrap().1, &50);
///
/// ```
    pub fn max<'a>(&'a self) -> Option<(&'a Range,&'a D)> {
        match self.root {
            Some(ref root) => Some(max_pair(root)),
            None => None
        }
    }

/// This function will return the hieght of the tree. An empty tree hash height 0, one with only
/// one elemente has height 1 etc.
/// # Examples
/// ```
/// extern crate memrange;
/// extern crate theban_interval_tree;
///
/// let mut t=theban_interval_tree::IntervalTree::<i32>::new();
/// assert_eq!(t.height(), 0);
/// t.insert(memrange::Range::new(2,2),3);
/// assert_eq!(t.height(), 1);
///
/// ```
    pub fn height(&self) -> usize {
        height(&self.root) as usize
    }

/// This function will return a read only iterator for all (key,value) pairs in the tree.
/// # Examples
/// ```
/// # let mut t=theban_interval_tree::IntervalTree::<i32>::new();
/// for (key,val) in t.iter() {
///     println!("{:?} -> {}",key,val)
/// }
///
/// ```
    pub fn iter(&self) -> RangePairIter<D>{
        RangePairIter::new(self, 0, 0xffff_ffff_ffff_ffff)
    }

/// This function will return a read only iterator for all (key,value) pairs between the two
/// bounds.
/// # Examples
/// ```
/// //[...]
/// # let mut t=theban_interval_tree::IntervalTree::<i32>::new();
/// for (key,val) in t.range(9, 100) {
///     println!("{:?} -> {}",key,val)
/// }
///
/// ```
    pub fn range(&self, min: u64, max: u64) -> RangePairIter<D>{
        RangePairIter::new(self, min, max)
    }

}

#[cfg(test)]
mod tests {
    use {memrange, rand};
    use node::is_interval_tree;

    fn random_range() -> memrange::Range {
        let offset = rand::random::<u64>()%50;
        let len: u64;
        len = rand::random::<u64>()%50;
        return memrange::Range::new(offset, offset+len)
    }

    #[test]
    fn test_fuzz(){
        let mut t = ::IntervalTree::<i32>::new();
        for _ in 1..5000 {
            let decision = rand::random::<bool>();
            let range = random_range();
            if  decision {
                t.insert(range, 1337);
                assert!(t.contains(range));
                assert!(is_interval_tree(&t.root));
            } else {
                t.delete(range);
                assert!(!t.contains(range));
                assert!(is_interval_tree(&t.root));
            };
        };
        return;
    }
}
