/*
 * @Date: 2023-02-06
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-02-06
 * @FilePath: /algorithm/rust/2331_evaluate_tree/evaluate_tree.rs
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

use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

impl Solution {
    pub fn evaluate_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match root.unwrap().as_ref().replace(TreeNode {
            val: 0,
            left: None,
            right: None,
        }) {
            mut x if x.val == 3 => {
                Solution::evaluate_tree(x.left.take()) && Solution::evaluate_tree(x.right.take())
            }
            mut x if x.val == 2 => {
                Solution::evaluate_tree(x.left.take()) || Solution::evaluate_tree(x.right.take())
            }
            x if x.val == 1 => true,
            _ => false,
        }
    }
}

fn main() {
    {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            }))),
        })));
        let ans = true;
        assert_eq!(Solution::evaluate_tree(root), ans);
    }

    {
        let root = Some(Rc::new(RefCell::new(TreeNode::new(0))));
        let ans = false;
        assert_eq!(Solution::evaluate_tree(root), ans);
    }
}
