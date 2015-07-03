use ::tree;
use ::node;
use ::node::Range;
use std::collections::Bound;

pub struct RangePairIter<'a,D:'a> {
    tree: &'a tree::IntervalTree<D>,
    range: Range,
    prev: Option<&'a Range>,
}

impl<'a, D:'a> RangePairIter<'a, D> {

    pub fn new(tree: &'a tree::IntervalTree<D>, lower: u64, upper: u64) -> RangePairIter<'a,D>{
        RangePairIter{tree: tree, range: Range::new(lower,upper), prev:None}
    }

    fn get_next_key_under(&mut self, root: &'a Box<node::Node<D>>) -> Option<(&'a Range,&'a D)>{
        let res = self.get_next_pair(root).and_then(|p| self.check_upper_bound(p));
        if let Some((key,_)) = res { self.prev = Some(key) }
        return res
    }

    fn get_next_pair(&mut self, root: &'a Box<node::Node<D>>) -> Option<(&'a Range, &'a D)>{
        match self.prev{
            None => self.get_lower_bound_pair(root),
            Some(key) => node::min_after::<D>(key, root)
        }
    }

    fn get_lower_bound_pair(&self, root: &'a Box<node::Node<D>>) -> Option<(&'a Range, &'a D)>{
            node::search_pair(&Range::new(self.range.min, self.range.min), root).or_else(|| node::min_after(&Range::new(self.range.min, self.range.min), root))
    }

    fn check_upper_bound(&self, current: (&'a Range, &'a D)) -> Option<(&'a Range, &'a D)> {
        return if current.0.max <= self.range.max { Some(current) } else { None };
    }
}

impl<'a, D:'a> Iterator for RangePairIter<'a, D> {

    type Item = (&'a Range,&'a D);

    fn next(&mut self) -> Option<(&'a Range,&'a D)> {
        self.tree.root.as_ref().map_or(None,|node| self.get_next_key_under(node))
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
    let init_key = Range::new(0,0);
    let mut iter = RangePairIter::<i32>{tree: &tree, prev: Some(&init_key), range: Range::new(0,100)};
    assert_eq!(iter.next().expect("should have a few values").0, &Range::new(1 ,1 ));
    assert_eq!(iter.next().expect("should have a few values").0, &Range::new(3 ,3 ));
    assert_eq!(iter.next().expect("should have a few values").0, &Range::new(10,10));
    assert_eq!(iter.next().expect("should have a few values").0, &Range::new(13,13));
    assert_eq!(iter.next().expect("should have a few values").0, &Range::new(17,17));
    assert_eq!(iter.next().expect("should have a few values").0, &Range::new(18,18));
    assert!(iter.next().is_none());

    let mut iter = RangePairIter::new(&tree, 3, 17);
    assert_eq!(iter.next().expect("should have a few values").0, &Range::new(3 ,3 ));
    assert_eq!(iter.next().expect("should have a few values").0, &Range::new(10,10));
    assert_eq!(iter.next().expect("should have a few values").0, &Range::new(13,13));
    assert_eq!(iter.next().expect("should have a few values").0, &Range::new(17,17));
    assert!(iter.next().is_none());
}
