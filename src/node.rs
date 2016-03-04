use std::cmp;
use std::cmp::Ordering;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Hash)]
pub struct Range{
    pub min: u64,
    pub max: u64
}

impl Range {
    pub fn new(min: u64, max: u64) -> Range{
        assert!(min <= max);
        return Range{min: min, max: max}
    }

    pub fn intersect(&self, other: &Range) -> bool{
        cmp::max(self.min,other.min) <= cmp::min(self.max,other.max)
    }

    pub fn len(&self) -> u64{
        return self.max-self.min
    }
}

impl Ord for Range {
    fn cmp(&self, other: &Self) -> Ordering {
        let first_cmp = self.min.cmp(&other.min);
        if first_cmp == Ordering::Equal { return self.max.cmp(&other.max) }
        return first_cmp
    }
}

pub struct Node<D> {
    pub key: Range,
    pub data: D,
    height: u32,
    max: u64,
    pub left: Option<Box<Node<D>>>,
    pub right:Option<Box<Node<D>>>,
}

impl<D> Node<D> {
    pub fn new(key: Range, data: D) -> Node<D>{
        Node::<D>{key: key, data: data, height: 1, max: key.max, left: None, right: None}
    }

    pub fn right_subtree_relevant(&self, range: &Range) -> bool{
        return range.max >= self.key.min
    }

    pub fn left_subtree_relevant(&self, range: &Range) -> bool{
        return self.max >= range.min
    }
}

pub fn height<D>(node: &Option<Box<Node<D>>>) -> u32  {
    return node.as_ref().map_or(0, |succ| succ.height)
}


fn subtree_max<D>(node: &Option<Box<Node<D>>>) -> u64 {
    return node.as_ref().map_or(0, |succ| succ.max)
}

/// Perform a single right rotation on this (sub) tree
fn rotate_right<D>(mut root: Box<Node<D>>) -> Box<Node<D>>{
    let mut new_root_box = root.left.take().expect("Avl broken");
    root.left = new_root_box.right.take();
    update_height(&mut root);
    new_root_box.right = Some(root);
    update_height(&mut new_root_box);
    return new_root_box
}

/// Perform a single left rotation on this (sub) tree
fn rotate_left<D>(mut root: Box<Node<D>>) -> Box<Node<D>>{
    let mut new_root_box = root.right.take().expect("Avl broken");
    root.right = new_root_box.left.take();
    update_height(&mut root);
    new_root_box.left = Some(root);
    update_height(&mut new_root_box);
    return new_root_box
}

/// Performs a rotation that counteracts the fact that the left successor is too high
fn rotate_left_successor<D>(mut root: Box<Node<D>>) -> Box<Node<D>> {
    let left = root.left.take().expect("Interval broken");
    if height(&left.left) < height(&left.right) {
        let rotated = rotate_left(left);
        root.left = Some(rotated);
        update_height(&mut root);
    }
    else{
        root.left = Some(left);
    }
    rotate_right(root)
}

/// Performs a rotation that counteracts the fact that the right successor is too high
fn rotate_right_successor<D>(mut root: Box<Node<D>>) -> Box<Node<D>> {
    let right = root.right.take().expect("Interval broken");
    if height(&right.left) > height(&right.right) {
        let rotated = rotate_right(right);
        root.right = Some(rotated);
        update_height(&mut root);
    }
    else {
        root.right = Some(right)
    }
    rotate_left(root)
}

fn diff_of_successors_height<D>(root: &Box<Node<D>>) -> i32 {
    let l = height(&root.left);
    let r = height(&root.right);
    (l as i32) - (r as i32)
}


/// Apply all necessary rotations on root. 
fn rotate_if_necessary<D>(root: Box<Node<D>>) -> Box<Node<D>> {
    let diff  = diff_of_successors_height(&root);
    if -1 <= diff && diff <= 1 {return root}
    match diff{
        2 => rotate_left_successor::<D>(root),
        -2 => rotate_right_successor::<D>(root),
        _ => unreachable!()
    }
}

/// update the cached height of root. To call this function make sure that the cached values of
/// both children of root ar up to date.
fn update_height<D>(root: &mut Node<D>){
    root.height = cmp::max( height(&root.left), height(&root.right) )+1;
    root.max = cmp::max(subtree_max(&root.left), cmp::max(subtree_max(&root.right), root.key.max));
}

/// recursively insert the (key,data) pair into the given optional succesor and return its new
/// value
fn insert_in_successor<D>(key: Range, data: D, successor: Option<Box<Node<D>>>)->Option<Box<Node<D>>> {
            Some(match successor {
                Some(succ) => insert(key, data, succ),
                None =>Box::new(Node::new(key, data))
            })
}

/// Inserts the given data under the key in the tree root. It will replace old data stored
/// under this key if it was allready used in the tree. The resulting tree will be returned (its
/// root may now differ due to rotations, thus the old root is moved into the function)
pub fn insert<D>(key: Range, data: D, mut root: Box<Node<D>>) -> Box<Node<D>>{
    match root.key.cmp(&key) {
        Ordering::Equal => { root.data  = data; return root },
        Ordering::Less =>    root.right = insert_in_successor(key, data, root.right.take()),
        Ordering::Greater => root.left  = insert_in_successor(key,data, root.left.take())
    }
    update_height(&mut *root);
    return rotate_if_necessary(root)
}

/// returns a read only reference to the data stored under key in the tree given by root
pub fn search<'a, D>(key: &Range, root: &'a Box<Node<D>>) -> Option<&'a D>{
    search_pair(key,root).map(|(_,v)| v )
}

/// returns a read only reference paie to the data stored under key in the tree given by root
pub fn search_pair<'a,D>(key: &Range, root: &'a Box<Node<D>>) -> Option<(&'a Range,&'a D)>{
    match root.key.cmp(key) {
        Ordering::Equal => Some((&root.key, &root.data)),
        Ordering::Less => root.right.as_ref().map_or(None, |succ| search_pair(key, succ)),
        Ordering::Greater => root.left.as_ref().map_or(None, |succ| search_pair(key, succ))
    }
}


/// returns true iff key is stored in the tree given by root
fn contains<D>(key: &Range, root: &Box<Node<D>> ) -> bool  {
    search(key,root).is_some()
}


///returns the smallest key and value after the given key.
pub fn min_after<'a,D>(key: &Range, root: &'a Box<Node<D>>) -> Option<(&'a Range,&'a D)> {
    match root.key.cmp(key){
        Ordering::Equal =>  root.right.as_ref().map_or(None, |succ| Some(min_pair(succ))),
        Ordering::Less =>   root.right.as_ref().map_or(None, |succ| min_after(key, succ)),
        Ordering::Greater => {
            match root.left {
                Some(ref succ) => min_after(key, &succ).or( Some((&root.key,&root.data)) ),
                None => Some((&root.key, &root.data))
            }
        }
    }
}

///returns the minimal key,value pair within this tree
pub fn min_pair<D>(root: &Box<Node<D>>) -> (&Range,&D) {
    root.left.as_ref().map_or((&root.key,&root.data), min_pair)
}

///returns the maximal key,value pair within this tree
pub fn max_pair<D>(root: &Box<Node<D>>) -> (&Range,&D) {
    root.right.as_ref().map_or((&root.key,&root.data), max_pair)
}

///returns the minimal value within this tree
pub fn min<D>(root: &Box<Node<D>>) -> &D {
    root.left.as_ref().map_or(&root.data, min)
}

///returns the minimal value within this tree
pub fn max<D>(root: &Box<Node<D>>) -> &D {
    root.right.as_ref().map_or(&root.data, max)
}

//will update_heights and rotate the node if necessary, returns the rotated node
fn updated_node<D>(mut root: Box<Node<D>>) -> Box<Node<D>> {
    update_height(&mut root);
    rotate_if_necessary(root)
}

//Performs recursive `drop_and_get_min` if a left  since a successor is available
fn drop_min_from_left<D>(mut root : Box<Node<D>>, left: Box<Node<D>>) -> (Option<Box<Node<D>>>,Box<Node<D>>) {
    let (new_left, min) =  drop_min(left);
    root.left = new_left;
    (Some(updated_node(root)),min)
}

//Finds the minimal value below root and returns a new (optional) tree where the minimal value has been
//removed and the (optional) minimal node as tuple (new_tree, min);
fn drop_min<D>(mut root: Box<Node<D>>) -> (Option<Box<Node<D>>>, Box<Node<D>>) {
    match root.left.take() {
        Some(left) => drop_min_from_left(root, left),
        None => (root.right.take(), root)
    }
}

//Return a new Interval tree, as the combination of two subtrees with max(l) <= min(r)
fn combine_two_subtrees<D>(l: Box<Node<D>>, r: Box<Node<D>>) -> Box<Node<D>>{
    let (remaining_tree, min) = drop_min(r);
    let mut new_root = min;
    new_root.left = Some(l);
    new_root.right = remaining_tree;
    updated_node(new_root)
}

//Return a new Interval tree, where the root has been removed
fn delete_root<D>(mut root: Box<Node<D>>) -> Option<Box<Node<D>>> {
    match ( root.left.take(), root.right.take() ) {
        ( None,     None)    => None,
        ( Some(l),  None)    => Some(l),
        ( None,     Some(r)) => Some(r),
        ( Some(l),  Some(r)) => Some(combine_two_subtrees(l,r))
    }
}


// will delete `key` from the tree `root`. Returns either `Some` tree or if the resilting tree is
// empty: None.
//
//
pub fn delete<D>(key: Range, mut root: Box<Node<D>>) -> Option<Box<Node<D>>>{
    match root.key.cmp(&key){
        Ordering::Equal =>  return delete_root(root),
        Ordering::Less => {
            if let Some(succ) = root.right.take() {
                root.right = delete(key, succ);
                return Some(updated_node(root))
            }
        },
        Ordering::Greater => {
            if let Some(succ) = root.left.take() {
                root.left =  delete(key, succ);
                return Some(updated_node(root))
            }
        }
    }
    return Some(root);
}

fn simple_tree(size: i32) -> Box<Node<i32>> {
    let mut t = Box::new(Node::<i32>{key: Range::new(1,1), data: 1337, height: 0, max: 1, left:None, right: None});
    for x in 2..size+1 {
        t = insert(Range::new(x as u64, x as u64 ),1337+x-1,t)
    }
    t
}

fn is_sorted_left<D>(node: &Box<Node<D>>) -> bool {
    node.left.as_ref().map_or(true, |succ| succ.key < node.key)
}

fn is_sorted_right<D>(node: &Box<Node<D>>) -> bool {
    node.right.as_ref().map_or(true, |succ| succ.key > node.key)
}

fn is_interval_node<D>(node: &Box<Node<D>>) -> bool {
    let sorted = is_sorted_left(node) && is_sorted_right(node);
    let balanced = node.height == cmp::max(height(&node.left),height(&node.right))+1;
    let proper_max = node.max == cmp::max(subtree_max(&node.left), cmp::max(subtree_max(&node.right), node.key.max));
    return sorted && balanced && proper_max;
}

pub fn is_interval_tree<D>(root: &Option<Box<Node<D>>>) -> bool {
    (*root).as_ref().map_or(true, is_interval_node)
}

#[test]
fn simple_tree_operations() {
    let mut t = Box::new(Node::<i32>{key: Range::new(3,3), data: 4, max:3, height: 2,
        left: Some(Box::new(Node::<i32>{key: Range::new(2,2), data: 5, height:1, max: 2, left: None, right: None})), 
        right: None});
    assert!(is_interval_node(&t));
    assert!( contains::<i32>(&Range::new(3,3),&t) );
    assert!( contains::<i32>(&Range::new(2,2),&t) );
    assert!( !contains::<i32>(&Range::new(6,6),&t) );
    assert!( !contains::<i32>(&Range::new(4,4),&t) );
    t = insert::<i32>(Range::new(4,4),7, t);
    t = insert::<i32>(Range::new(5,5),7, t);
    t = insert::<i32>(Range::new(6,6),8, t);
    assert!(  contains::<i32>(&Range::new(4,4),&t) );
    assert!(  contains::<i32>(&Range::new(6,6),&t) );
    assert!( !contains::<i32>(&Range::new(7,7),&t) );
}

#[test]
fn rotations_on_tree(){ 
    let mut t = Box::new(Node::<i32>{key: Range::new(1,1), data: 1337, height: 1, max: 1, left: None, right: None});
    for i in 2..255 {
        t = insert::<i32>(Range::new(i,i),1337, t);
        assert!(is_interval_node(&t));
    }
    //check that the tree is indeed balanced
    assert!(height(&Some(t)) <= 8);
}

#[test]
fn test_drop_min(){
    let mut t = simple_tree(3);
    let (maybe_tree,min) = drop_min(t);
    t = maybe_tree.expect("failure to get tree for first min delete");
    assert!(is_interval_node(&t));
    assert!( min.key == Range::new(1,1));
    assert!(!contains::<i32>(&Range::new(1,1),&t));
    assert!( contains::<i32>(&Range::new(2,2),&t));
    assert!( contains::<i32>(&Range::new(3,3),&t));

    let (maybe_tree,min) = drop_min(t);
    t = maybe_tree.expect("failure to get tree for second min delete");
    assert!(is_interval_node(&t));
    assert!( min.key == Range::new(2,2));
    assert!(!contains::<i32>(&Range::new(1,1),&t));
    assert!(!contains::<i32>(&Range::new(2,2),&t));
    assert!( contains::<i32>(&Range::new(3,3),&t));

    let (maybe_tree,min) = drop_min(t);
    assert!( maybe_tree.is_none() );
    assert!( min.key == Range::new(3,3));
}

#[test]
fn test_drop_root(){
    let mut t = simple_tree(3);
    let maybe_tree = delete_root(t);
    t = maybe_tree.expect("failure to get tree for first root drop");
    assert!(is_interval_node(&t));
    assert!( t.height == 2);
    assert!( contains::<i32>(&Range::new(1,1),&t));
    assert!(!contains::<i32>(&Range::new(2,2),&t));
    assert!( contains::<i32>(&Range::new(3,3),&t));

    let maybe_tree = delete_root(t);
    t = maybe_tree.expect("failure to get tree for second root drop");
    assert!(is_interval_node(&t));
    assert!( contains::<i32>(&Range::new(1,1),&t));
    assert!(!contains::<i32>(&Range::new(2,2),&t));
    assert!(!contains::<i32>(&Range::new(3,3),&t));

    let maybe_tree = delete_root(t);
    assert!( maybe_tree.is_none() );
}

#[test]
fn test_delete(){
    let mut t = simple_tree(10);
    for i in 1..10 {
        assert!(contains::<i32>(&Range::new(i,i),&t));
        let maybe_tree = delete(Range::new(i,i),t);
        t = maybe_tree.expect("failure to get tree for delete");
        assert!(!contains::<i32>(&Range::new(i,i),&t));
        assert!(is_interval_node(&t));
    }
    assert!(contains::<i32>(&Range::new(10,10),&t));
    let maybe_tree = delete(Range::new(10,10),t);
    assert!(maybe_tree.is_none());
}

#[test] 
fn test_min_max() {
    let mut t = simple_tree(50);
    assert_eq!(min(&t),&1337);
    assert_eq!(max(&t),&(1337+50-1));
    assert_eq!(max_pair(&t).0,&Range::new(50,50));
    assert_eq!(min_pair(&t).0,&Range::new(1,1));
}

#[test]
fn test_min_after(){
    let t = simple_tree(50);
    for old_key in 0..55 {
        println!("trying value: {}", old_key);
        match min_after(&Range::new(old_key,old_key),&t) {
            Some((k,_d)) => assert_eq!(k, &(Range::new(old_key+1,old_key+1))),
            None => assert!(old_key >= 50)
        }
    }
}
