extern crate interval_tree;
extern crate rand;
extern crate time;

use time::PreciseTime;
use interval_tree::Range;
use std::cmp;
use std::collections::BTreeSet;

#[test]
fn test_getters(){
    let data = 1337;
    let mut t = interval_tree::IntervalTree::<i32>::new();
    t.insert(Range::new(1,1), data);
    t.insert(Range::new(2,2), data+1);
    t.insert(Range::new(3,3), data+2);
    assert!(t.get_or(Range::new(1,1), &0) == &data);
    assert!(t.get_or(Range::new(2,2), &0) == &(data+1));
    assert!(t.get_or(Range::new(3,3), &0) == &(data+2));
    assert!(t.get_or(Range::new(4,4), &0) == &0);
    assert!(t.get(Range::new(4,4)) == None);
}

#[test]
fn test_contains(){
    let data = 1337;
    let mut t = interval_tree::IntervalTree::<i32>::new();
    t.insert(Range::new(1,1), data);
    t.insert(Range::new(2,2), data+1);
    t.insert(Range::new(3,3), data+2);
    assert!(!t.contains(Range::new(0,0)));
    assert!( t.contains(Range::new(1,1)));
    assert!( t.contains(Range::new(2,2)));
    assert!( t.contains(Range::new(3,3)));
    assert!(!t.contains(Range::new(4,4)));
}

#[test]
fn test_empty(){
    let data = 1337;
    let mut t = interval_tree::IntervalTree::<i32>::new();
    assert!(t.empty());
    t.insert(Range::new(1,1), data);
    t.insert(Range::new(2,2), data+1);
    t.insert(Range::new(3,3), data+2);
    assert!(!t.empty());
}

#[test]
fn test_delete(){
    let data = 1337;
    let mut t = interval_tree::IntervalTree::<i32>::new();
    t.insert(Range::new(1,1), data);
    t.insert(Range::new(2,2), data+1);
    t.insert(Range::new(3,3), data+2);
    t.delete(Range::new(1,1));
    assert!(!t.contains(Range::new(1,1)));
    assert!( t.contains(Range::new(2,2)));
    assert!( t.contains(Range::new(3,3)));
    t.delete(Range::new(2,2));
    assert!(!t.contains(Range::new(1,1)));
    assert!(!t.contains(Range::new(2,2)));
    assert!( t.contains(Range::new(3,3)));
    t.delete(Range::new(3,3));
    assert!(!t.contains(Range::new(1,1)));
    assert!(!t.contains(Range::new(2,2)));
    assert!(!t.contains(Range::new(3,3)));
    assert!(t.empty());
}

#[test]
fn test_perfomance(){
    let mut t = interval_tree::IntervalTree::<i32>::new();
    let data = 1337;
    let start = PreciseTime::now();
    for _ in 1..10000 {
        t.insert(Range::new(1,1), data);
        t.insert(Range::new(20000,20000), data+1);
        t.delete(Range::new(1,1));
        t.delete(Range::new(20000,20000));
    }
    let end = PreciseTime::now();
    let diff_simple = start.to(end).num_milliseconds();
    for x in 5..2000 {
        t.insert(Range::new(x,x), data);
    }

    let start_2 = PreciseTime::now();
    for _ in 1..10000 {
        t.insert(Range::new(1,1), data);
        t.insert(Range::new(20000,20000), data+1);
        t.delete(Range::new(1,1));
        t.delete(Range::new(20000,20000));
    }
    let end_2 = PreciseTime::now();
    let diff_full = start_2.to(end_2).num_milliseconds();
    assert!(diff_full < diff_simple * 13); //log time 
}

#[test]
fn test_min(){
    let mut t = interval_tree::IntervalTree::<i32>::new();
    assert!{t.min().is_none()};
    t.insert(Range::new(50,50), 1337);
    assert_eq!{t.min().expect("get 1 min"),(&Range::new(50,50),&1337)};
    t.insert(Range::new(49,49),1338);
    assert_eq!{t.min().expect("get 2 min"),(&Range::new(49,49),&1338)};
    t.insert(Range::new(47,47),1339);
    assert_eq!{t.min().expect("get 3 min"),(&Range::new(47,47),&1339)};
    t.insert(Range::new(48,48),1340);
    assert_eq!{t.min().expect("get 4 min"),(&Range::new(47,47),&1339)};
}

#[test]
fn test_iter(){
    let mut t = interval_tree::IntervalTree::<i32>::new();
    t.insert(Range::new(32,32),1337);
    t.insert(Range::new(34,34),1338);
    t.insert(Range::new(36,36),1339);
    t.insert(Range::new(38,38),1340);
    for (i,pair) in t.iter().enumerate() {
        let (k,v) = pair;
        println!("{:?}, {}",k,v);
        let key = (i as u64)*2 +32;
        assert_eq!(k,&Range::new(key,key));
        assert_eq!(v,&((i as i32)+1337));
    }

}

#[test]
fn test_range_iter(){
    let mut t = interval_tree::IntervalTree::<i32>::new();
    t.insert(Range::new(32,32),1337);
    t.insert(Range::new(34,34),1338);
    t.insert(Range::new(36,36),1339);
    t.insert(Range::new(38,38),1340);
    for (i,pair) in t.range(34, 36).enumerate() {
        let (k,v) = pair;
        println!("{:?}, {}",k,v);
        let key = (i as u64)*2 +34;
        assert_eq!(k,&Range::new(key,key));
        assert_eq!(v,&((i as i32)+1338));
        assert!(i<2);
    }

}

#[test]
fn test_range_iter_non_pointwise(){
    let mut t = interval_tree::IntervalTree::<i32>::new();
    t.insert(Range::new(3,8),1337);
    t.insert(Range::new(6,10),1338);
    t.insert(Range::new(12,36),1339);
    t.insert(Range::new(32,40),1340);
    assert_eq!(t.range(9,14).map(|(&k,&v)| k.min).collect::<Vec<u64>>(), vec![6,12])
}

fn random_range() -> Range {
    let offset = rand::random::<u64>();
    let len: u64;
    if rand::random::<bool>() {
        len = cmp::min(rand::random::<u64>()%500, 0xff_ff_ff_ff_ff_ff_ff_ff - offset)
    } else {
        len = rand::random::<u64>()%(0xff_ff_ff_ff_ff_ff_ff_ff - offset)
    }

    return Range::new(offset, offset+len)
}

#[test]
fn test_range_iter_nontrivial(){
    let mut set = BTreeSet::<Range>::new();
    let mut t = interval_tree::IntervalTree::<i32>::new();
    for _ in 1..5000 {
        let decision = rand::random::<bool>();
        let range = random_range();
        if  decision {
            set.insert(range);
            t.insert(range, 1337);
            assert!(t.contains(range));
            //assert!(t.test_interval_tree());
        } else {
            set.remove(&range);
            t.delete(range);
            assert!(!t.contains(range));
            //assert!(t.test_interval_tree());
        };
    let query = random_range();
    let should = set.iter().filter(|&r| query.intersect(r)).collect::<Vec<&Range>>();
    let is = t.range(query.min, query.max).map(|(r,_)| r).collect::<Vec<&Range>>();
    assert_eq!(should, is);
    };
}
