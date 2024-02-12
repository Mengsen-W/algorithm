/*
 * @Date: 2024-02-12
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-02-13
 * @FilePath: /algorithm/rust/145_postorder_traversal/postorder_traversal.rs
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
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];
        let mut stack = VecDeque::new();
        stack.push_back(root.map(|x| (x, false)));
        while let Some(x) = stack.pop_back() {
            match x {
                Some((node, true)) => res.push(node.borrow().val),
                Some((node, false)) => {
                    stack.push_back(Some((node.clone(), true)));
                    stack.push_back(node.borrow().right.clone().map(|x| (x, false)));
                    stack.push_back(node.borrow().left.clone().map(|x| (x, false)));
                }
                _ => (),
            }
        }
        res
    }
}

fn main() {
    let tests = vec![
        (
            Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                    right: None,
                }))),
            }))),
            vec![3, 2, 1],
        ),
        (None, vec![]),
        (Some(Rc::new(RefCell::new(TreeNode::new(1)))), vec![1]),
    ];

    for (root, ans) in tests {
        assert_eq!(Solution::postorder_traversal(root), ans);
    }
}
