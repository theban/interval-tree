#[cfg(test)]
extern crate rand;
extern crate memrange;

mod node;
pub mod tree;
mod iterators;
pub use tree::IntervalTree;
pub use iterators::RangePairIter;
