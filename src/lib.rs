#![crate_type = "lib"]
#![feature(test)]
#![feature(collections_bound)]
#![feature(rand)]


mod node;
pub use node::Range;
pub mod tree;
mod iterators;
pub use tree::IntervalTree;
pub use iterators::RangePairIter;
