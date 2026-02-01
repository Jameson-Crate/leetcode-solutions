// 105. Construct Binary Tree from Preorder and Inorder Traversal
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
use std::collections::HashMap;
impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn builder(preorder: &mut std::slice::Iter<i32>, index_map: &HashMap<i32, usize>, range: (isize, isize)) -> Option<Rc<RefCell<TreeNode>>> {
            if range.0 <= range.1 {
                if let Some(&val) = preorder.next() {
                    if let Some(&i) = index_map.get(&val) {
                        return Some(Rc::new(RefCell::new(TreeNode {
                            val,
                            left: builder(preorder, index_map, (range.0, i as isize - 1)),
                            right: builder(preorder, index_map, (i as isize + 1, range.1))
                        })))
                    }
                }
            }
            None
        }
        let hm = inorder.iter().enumerate().map(|(i, &val)| (val, i)).collect::<HashMap<_, _>>();
        return builder(&mut preorder.iter(), &hm, (0, preorder.len() as isize - 1));
    }
}
