// 1448. Count Good Nodes in Binary Tree
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
use std::collections::VecDeque;
use std::cmp;
impl Solution {
    pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut ans = 0;
        let mut q: VecDeque<(Option<Rc<RefCell<TreeNode>>>, i32)> = VecDeque::new();
        q.push_back((root.clone(), i32::MIN));
        while !q.is_empty() {
            if let Some((node, val)) = q.pop_front() {
                if let Some(node_rc) = node {
                    let inner = node_rc.borrow();
                    if inner.val >= val {
                        ans += 1;
                    }
                    q.push_back((inner.left.clone(), cmp::max(inner.val, val)));
                    q.push_back((inner.right.clone(), cmp::max(inner.val, val)));
                }
            }
        }
        ans
    }
}
