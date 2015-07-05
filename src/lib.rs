#![crate_type = "lib"]
#![feature(test)]
#![feature(collections_bound)]
#![feature(rand)]


mod node;
pub use node::Range;
pub mod tree;
mod iterators;
mod db;
pub use tree::IntervalTree;
pub use db::DB;
