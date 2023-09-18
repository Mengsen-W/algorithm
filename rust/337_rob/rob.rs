/*
 * @Date: 2023-09-18
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-09-18
 * @FilePath: /algorithm/rust/337_rob/rob.rs
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
    pub fn sub_rob(node: &Option<Rc<RefCell<TreeNode>>>) -> [i32; 2] {
        if let Some(n) = node {
            let dp_l = Self::sub_rob(&n.borrow().left);
            let dp_r = Self::sub_rob(&n.borrow().right);
            [
                dp_l[1] + dp_r[1],
                (dp_l[0] + dp_r[0] + n.borrow().val).max(dp_l[1] + dp_r[1]),
            ]
        } else {
            [0; 2]
        }
    }

    pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::sub_rob(&root)[1]
    }
}

fn main() {
    let tests = vec![
        (
            Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                }))),
            }))),
            7,
        ),
        (
            Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                }))),
            }))),
            9,
        ),
    ];

    for (node, ans) in tests {
        assert_eq!(Solution::rob(node), ans);
    }
}
