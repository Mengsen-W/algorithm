/*
 * @Date: 2023-08-25
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-08-25
 * @FilePath: /algorithm/rust/1448_good_nodes/good_nodes.rs
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
    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, max: i32) -> i32 {
        if node.is_none() {
            return 0;
        }
        let val = node.as_ref().unwrap().borrow().val;
        let n = val.max(max);
        let l = Self::dfs(&node.as_ref().unwrap().borrow().left, n);
        let r = Self::dfs(&node.as_ref().unwrap().borrow().right, n);
        l + r + if val >= max { 1 } else { 0 }
    }

    pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::dfs(&root, i32::MIN)
    }
}

fn main() {
    let tests = vec![
        (
            Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
                }))),
            }))),
            4,
        ),
        (
            Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
                }))),
                right: None,
            }))),
            3,
        ),
        (Some(Rc::new(RefCell::new(TreeNode::new(1)))), 1),
    ];

    for (root, ans) in tests {
        assert_eq!(Solution::good_nodes(root), ans)
    }
}
