#![crate_type = "lib"]
#![feature(test)]

mod node;
mod range;
pub use range::Range;
pub mod tree;
mod iterators;
pub use tree::IntervalTree;
pub use iterators::RangePairIter;
