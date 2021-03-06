//! Data structures

mod disjoint_set;
pub use disjoint_set::DisjointSet;

mod monotonic_queue;
pub use monotonic_queue::MonotonicQueue;

mod prefix_sum;
pub use prefix_sum::PrefixSum1D;

mod fenwick_tree;
pub use fenwick_tree::FenwickTree;

mod sparse_table;
pub use sparse_table::SparseTable;
