// https://leetcode.com/problems/search-in-a-binary-search-tree/
// source code: https://github.com/inrust/Rust-Programming-in-Action/blob/master/algorithm/dfs-and-bfs/leetcode_700_search-in-a-binary-search-tree/src/main.rs

use std::rc::Rc;
use std::cell::RefCell;

/ #[derive(Debug, PartialEq, Eq)]

pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}

// Solution
struct Solution{}
impl Solution {
    pub fn search_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        let mut r = root.clone();
        while let Some(node) = r {
            if node.borrow().val == val { return Some(node); }
            if node.borrow().val > val {
                r = node.borrow().left.clone();
            } else {
                r = node.borrow().right.clone();
            }
        }
        None
    }
}