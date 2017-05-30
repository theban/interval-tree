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
