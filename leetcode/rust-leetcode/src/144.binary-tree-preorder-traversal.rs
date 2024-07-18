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
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut stack = Vec::new();
        let mut preorder = Vec::new();
        stack.push(root);
        while let Some(node) = stack.pop() {
            match node {
                None => {},
                Some(node) => {
                    let node = node.borrow();
                    preorder.push(node.val);
                    stack.push(node.right.clone());
                    stack.push(node.left.clone());
                }
            }
        }
        preorder
    }
}

