# // https://leetcode.com/problems/search-in-a-binary-search-tree/
# https://walkccc.me/LeetCode/problems/0700/

class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

class Solution:
#   inputs
#   이진 검색 트리(BST)의 root
#   Int number val

#   Output
#   부분 트리를 반환
#   Or null
  def searchBST(self, root: Optional[TreeNode], val: int) -> Optional[TreeNode]:
    # condition) end of recursion  
    if not root:
        return None

    # preprocess) root node == vale
    if root.val == val:
       return root 

    # val is root's left side
    if root.val > val:
       return self.searchBST(root.left, val);

    # else
    # val is root's right side
    return self.searchBST(root.right, val)