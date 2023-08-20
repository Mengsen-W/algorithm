/*
 * @Date: 2023-08-20
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-08-20
 * @FilePath: /algorithm/rust/2236_check_tree/check_tree.rs
 */

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
struct Solution;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn check_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match root {
            Some(node) => match (node.borrow().left.as_ref(), node.borrow().right.as_ref()) {
                (Some(left), Some(right)) => {
                    left.borrow().val + right.borrow().val == node.borrow().val
                }
                _ => true,
            },
            None => true,
        }
    }
}

fn main() {
    let tests = vec![
        (
            Some(Rc::new(RefCell::new(TreeNode {
                val: 10,
                left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
            }))),
            true,
        ),
        (
            Some(Rc::new(RefCell::new(TreeNode {
                val: 5,
                left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            }))),
            false,
        ),
    ];

    for (root, ans) in tests {
        assert_eq!(Solution::check_tree(root), ans);
    }
}
