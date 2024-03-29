// https://leetcode.com/problems/diameter-of-binary-tree/description/
// https://github.com/doocs/leetcode/blob/main/solution/0500-0599/0543.Diameter%20of%20Binary%20Tree/README_EN.md

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
    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, res: &mut i32) -> i32 {
        if root.is_none() {
            return 0;
        }
        let root = root.as_ref().unwrap().as_ref().borrow();
        let left = Self::dfs(&root.left, res);
        let right = Self::dfs(&root.right, res);
        *res = (*res).max(left + right);
        left.max(right) + 1
    }

    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut res = 0;
        Self::dfs(&root, &mut res);
        res
    }
}

fn main(){
    // GG generating test case: root nodes
    // [1,2,3,4,5]
    let root= Some(Rc::new(RefCell::new(
        // ERR: IDK
        TreeNode { val: (), left: (), right: () }
    )));
    let result= Solution::diameter_of_binary_tree(root);
    println!("{}", result);
}