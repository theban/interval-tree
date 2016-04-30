extern crate memrange;

use ::tree;
use self::memrange::Range;
use ::node::Node;

enum VisitingState {
    VisitLeft,
    VisitCenter, 
    VisitRight
}

pub struct RangePairIter<'a,D:'a> {
    range: Range,
    stack: Vec<(&'a Node<D>, VisitingState)>
}


impl<'a, D:'a> RangePairIter<'a, D> {

    pub fn new(tree: &'a tree::IntervalTree<D>, lower: u64, upper: u64) -> RangePairIter<'a,D>{
        let mut stack = Vec::with_capacity(tree.height());
        if let Some(ref root) = tree.root {
            stack.push( (&**root,VisitingState::VisitLeft) );
        }
        //RangePairIter{tree: tree, range: Range::new(lower,upper), stack: stack}
        RangePairIter{ range: Range::new(lower,upper), stack: stack}
    }

    pub fn visit_left(&mut self, node: &'a Node<D>) {
        //println!("left {:?}", node.key);
        match node.left {
            Some(ref lsucc) => {
                self.stack.push( (node, VisitingState::VisitCenter) );
                if node.left_subtree_relevant(&self.range) { 
                    self.stack.push( (&**lsucc, VisitingState::VisitLeft) ) 
                }
            },
            None => self.stack.push( (node, VisitingState::VisitCenter) )
        }
    }

    pub fn visit_right(&mut self, node: &'a Node<D>) {
        //println!("right {:?}", node.key);
        if !node.right_subtree_relevant(&self.range) { return }
        match node.right {
            Some(ref rsucc) => {
                self.stack.push((&**rsucc, VisitingState::VisitLeft));
            },
            None => return
        }
    }

    pub fn visit_center(&mut self, node: &'a Node<D>) -> Option<&'a Node<D>>{
        //println!("center {:?}", node.key);
        self.stack.push((node, VisitingState::VisitRight));
        if node.key.intersect(&self.range){ return Some(node) } else { return self.get_next_node() }
    }

    pub fn get_next_node(&mut self) -> Option<&'a Node<D>>{
        if let Some((node, state)) = self.stack.pop() {
            match state {
                VisitingState::VisitLeft => {self.visit_left(node); return self.get_next_node()},
                VisitingState::VisitRight => {self.visit_right(node); return self.get_next_node()}
                VisitingState::VisitCenter => return self.visit_center(node),
            }
        } else {
            return None
        }
    }
}

impl<'a, D:'a> Iterator for RangePairIter<'a, D> {

    type Item = (Range,&'a D);

    fn next(&mut self) -> Option<(Range,&'a D)> {
        self.get_next_node().map_or(None, |n| Some((n.key, &n.data)))
    }
}

#[test]
fn test_iterators(){
    let mut tree = tree::IntervalTree::<i32>::new();
    tree.insert(Range::new(18,18), 1337);
    tree.insert(Range::new(13,13), 1338);
    tree.insert(Range::new(17,17), 1339);
    tree.insert(Range::new(10,10), 1321);
    tree.insert(Range::new(1 ,1), 1321);
    tree.insert(Range::new(3 ,3), 1322);

    let iter = RangePairIter::new(&tree, 0, 1000);

    for (k,v) in iter {
        println!("{:?} {}",k,v);
    }

    let mut iter = RangePairIter::new(&tree, 0, 1000);
    assert_eq!(iter.next().expect("should have a few values").0, Range::new(1 ,1 ));
    assert_eq!(iter.next().expect("should have a few values").0, Range::new(3 ,3 ));
    assert_eq!(iter.next().expect("should have a few values").0, Range::new(10,10));
    assert_eq!(iter.next().expect("should have a few values").0, Range::new(13,13));
    assert_eq!(iter.next().expect("should have a few values").0, Range::new(17,17));
    assert_eq!(iter.next().expect("should have a few values").0, Range::new(18,18));
    assert!(iter.next().is_none());

    let mut iter = RangePairIter::new(&tree, 3, 17);
    assert_eq!(iter.next().expect("should have a few values").0, Range::new(3 ,3 ));
    assert_eq!(iter.next().expect("should have a few values").0, Range::new(10,10));
    assert_eq!(iter.next().expect("should have a few values").0, Range::new(13,13));
    assert_eq!(iter.next().expect("should have a few values").0, Range::new(17,17));
    assert!(iter.next().is_none());
}
