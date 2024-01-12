 // https://leetcode.com/problems/search-in-a-binary-search-tree/
//  https://walkccc.me/LeetCode/problems/0700/s

 public class TreeNode {
    int val;
    TreeNode left;
    TreeNode right;
    TreeNode() {}
    TreeNode(int val) { this.val = val; }
    TreeNode(int val, TreeNode left, TreeNode right) {
        this.val = val;
        this.left = left;
        this.right = right;
    }
}

public class Solution {
  public TreeNode searchBST(TreeNode root, int val) {
    if (root == null)
      return null;
    if (root.val == val)
      return root;
    if (root.val > val)
      return searchBST(root.left, val);
    return searchBST(root.right, val);
  }
}