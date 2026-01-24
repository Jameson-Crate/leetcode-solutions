# Definition for a binary tree node.
class TreeNode:
    def __init__(self, x):
        self.val = x
        self.left = None
        self.right = None

class Solution:
    def lowestCommonAncestor(self, root: 'TreeNode', p: 'TreeNode', q: 'TreeNode') -> 'TreeNode':
        # Invert the tree
        def traverse(node, prev, G):
            if node:
                G[node.val] = prev
                traverse(node.left, node.val, G)
                traverse(node.right, node.val, G)
        G = dict()
        traverse(root, None, G)

        # Traverse from each node till we find a visited
        visited = set()
        p = p.val
        q = q.val
        while p != None or q != None:
            if p != None:
                if p in visited:
                    return TreeNode(p)
                visited.add(p)
                p = G[p]

            if q != None:
                if q in visited:
                    return TreeNode(q)
                visited.add(q)
                q = G[q]
        return TreeNode(None)
