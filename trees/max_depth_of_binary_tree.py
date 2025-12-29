# 104. Maximum Depth of Binary Tree
from typing import Optional

# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

class Solution:
    def maxDepth(self, root: Optional[TreeNode]) -> int:
        if not root:
            return 0
            
        s = [(root, 1)]
        ans = 1
        while s:
            curr, d = s.pop()
            ans = max(ans, d)
            if curr.left:
                s.append((curr.left, d + 1))
            if curr.right:
                s.append((curr.right, d + 1))
        return ans
