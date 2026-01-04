# 572. Subtree of Another Tree
from typing import Optional

# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

class Solution:
    def isSubtree(self, root: Optional[TreeNode], subRoot: Optional[TreeNode]) -> bool:
        def same(t1, t2):
            if not t1 or not t2:
                return not t1 and not t2
            return t1.val == t2.val and same(t1.left, t2.left) and same(t1.right, t2.right)

        if same(root, subRoot):
            return True
        
        if root:
            return self.isSubtree(root.left, subRoot) or self.isSubtree(root.right, subRoot)
        return False
            