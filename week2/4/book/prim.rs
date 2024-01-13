use std::cmp::Reverse;
use std::collections::{BTreeMap, BinaryHeap};
use std::ops::Add;

type Graph<V, E>= BTreeMap<V, BTreeMap<V, E>>;

// https://github.com/TheAlgorithms/Rust/blob/master/src/graph/prim.rs