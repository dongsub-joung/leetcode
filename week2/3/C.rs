// https://leetcode.com/problems/range-sum-of-bst/description/
// https://knarfeh.com/2014/03/11/leetcode/Algorithms/leetcode-938-Range-Sum-of-BST/

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
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

use std::rc::Rc;
use std::cell::RefCell;
struct Solution{}

impl Solution {
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, l: i32, r: i32) -> i32 {
        if let Some(root) = root {
            let mut sum = 0;
            if l <= root.borrow().val && root.borrow().val <= r {
                sum += root.borrow().val;
            }
            
            if l < root.borrow().val {
                sum += Self::range_sum_bst(root.borrow().left.clone(), l, r);
            }
            if r > root.borrow().val {
                sum += Self::range_sum_bst(root.borrow().right.clone(), l, r);
            }
            return sum;
        }
        return 0;
    }
}