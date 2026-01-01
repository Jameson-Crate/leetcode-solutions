# 110. Balanced Binary Tree
from typing import Optional

# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right
        
class Solution:
    def isBalanced(self, root: Optional[TreeNode]) -> bool:
        def depth(node):
            if not node:
                return 0, True
            dl, bl = depth(node.left)
            dr, br = depth(node.right)
            return max(dl, dr) + 1, abs(dr - dl) < 2 and bl and br
        _, balanced = depth(root)
        return balanced