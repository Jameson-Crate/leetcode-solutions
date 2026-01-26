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
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut ans: Vec<Vec<i32>> = Vec::new();
        fn dfs(node: Option<Rc<RefCell<TreeNode>>>, depth: usize, ans: &mut Vec<Vec<i32>>) {
            if let Some(node_rc) = node {
                let inner = node_rc.borrow();
                while ans.len() <= depth {
                    ans.push(Vec::new());
                }
                ans[depth].push(inner.val);
                dfs(inner.left.clone(), depth + 1, ans);
                dfs(inner.right.clone(), depth + 1, ans);
            }
        }
        dfs(root, 0, &mut ans);
        return ans;
    }
}
