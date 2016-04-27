#![crate_type = "lib"]
#![feature(test)]

mod node;
pub mod tree;
mod iterators;
pub use tree::IntervalTree;
pub use iterators::RangePairIter;
