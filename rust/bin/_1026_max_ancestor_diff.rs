/*
 * @Date: 2023-04-18
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-04-18
 * @FilePath: /algorithm/rust/1026_max_ancestor_diff/max_ancestor_diff.rs
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
    pub fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, mut maximum: i32, mut minimum: i32) -> i32 {
            if let Some(node) = root {
                let val = node.borrow().val;
                maximum = maximum.max(val);
                minimum = minimum.min(val);
                let left = node.borrow_mut().left.take();
                let right = node.borrow_mut().right.take();
                (maximum - minimum)
                    .max(dfs(left, maximum, minimum).max(dfs(right, maximum, minimum)))
            } else {
                0
            }
        }
        let val = root.as_ref().unwrap().borrow().val;
        dfs(root, val, val)
    }
}

fn main() {
    {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 8,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 6,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 10,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 14,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(13)))),
                    right: None,
                }))),
            }))),
        })));
        let ans = 7;
        assert_eq!(Solution::max_ancestor_diff(root), ans);
    }

    {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 0,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                    right: None,
                }))),
            }))),
        })));
        let ans = 3;
        assert_eq!(Solution::max_ancestor_diff(root), ans);
    }
}
