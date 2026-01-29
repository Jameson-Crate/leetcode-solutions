// 98. Validate Binary Search Tree
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        // In order traversal and then linear check
        fn dfs(node: Option<Rc<RefCell<TreeNode>>>, vals: &mut Vec<i32>) {
            if let Some(node_rc) = node {
                let inner = node_rc.borrow();
                dfs(inner.left.clone(), vals);
                vals.push(inner.val);
                dfs(inner.right.clone(), vals);
            }
        }

        let mut vals = Vec::new();
        dfs(root, &mut vals);
        for i in 1..vals.len() {
            if vals[i] <= vals[i - 1] {
                return false
            }
        }
        true
    }
}
