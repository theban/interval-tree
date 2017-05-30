IntervalTree
============

A simple crate that implements a interval tree datastructure. An `IntervalTree` maps ranges of `u64` to any value. We can than use the tree to perform querys such as "what key/value pairs are intersecting the range (x,y)?" does "does the tree contain the range (X,Y)?". Insertion, deletion and lookup are in O(log(n)). Iterating over all m solutions to a query is in O(m*log(n)).

```rust
extern crate theban_interval_tree;
extern crate rand;
extern crate time;
extern crate memrange;

use memrange::Range;

fn main(){
    let data = 4221;
    let mut t = theban_interval_tree::IntervalTree::<i32>::new();

    assert!(t.empty());
    assert!{t.min().is_none()};

    t.insert(Range::new(1,1), data);
    t.insert(Range::new(2,2), data+1);
    t.insert(Range::new(3,3), data+2);

    assert_eq!{t.min().expect("get min"),(&Range::new(1,1),&data)};

    assert!(!t.empty());
    assert!(t.get_or(Range::new(1,1), &0) == &data);
    assert!(!t.contains(Range::new(0,0)));

    t.delete(Range::new(1,1));

    assert!(!t.contains(Range::new(1,1)));

    for (i,pair) in t.iter().enumerate() {
        //[...]
    }

    for (i,pair) in t.range(34, 36).enumerate() {
        //[...]
    }
}
```
