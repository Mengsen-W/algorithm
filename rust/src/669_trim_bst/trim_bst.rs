/*
 * @Date: 2022-09-10
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-09-10
 * @FilePath: /algorithm/669_trim_bst/trim_bst.rs
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
    pub fn trim_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        low: i32,
        high: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(r) = root {
            if r.borrow().val > high {
                return Self::trim_bst(r.borrow_mut().left.take(), low, high);
            }

            if r.borrow().val < low {
                return Self::trim_bst(r.borrow_mut().right.take(), low, high);
            }

            let left = Self::trim_bst(r.borrow_mut().left.take(), low, high);
            let right = Self::trim_bst(r.borrow_mut().right.take(), low, high);

            r.borrow_mut().left = left;
            r.borrow_mut().right = right;

            Some(r)
        } else {
            None
        }
    }
}

fn main() {
    {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        })));
        let low = 1;
        let right = 2;
        let ans = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        })));
        assert_eq!(Solution::trim_bst(root, low, right), ans);
    }

    {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 0,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                    right: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
        })));
        let low = 1;
        let right = 3;
        let ans = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                right: None,
            }))),
            right: None,
        })));
        assert_eq!(Solution::trim_bst(root, low, right), ans);
    }
}
