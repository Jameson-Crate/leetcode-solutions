# 543. Diameter of Binary Tree
from typing import Optional

# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def diameterOfBinaryTree(self, root: Optional[TreeNode]) -> int:
        ans = 0
        def height(node):
            nonlocal ans
            if not node:
                return 0
            hl = height(node.left)
            hr = height(node.right)
            ans = max(ans, hl + hr)
            return max(hl, hr) + 1
        height(root)
        return ans
