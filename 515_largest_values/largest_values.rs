/*
 * @Date: 2022-06-24
 * @LastEditors: Mengsen Wang mengsen_wang@163.com
 * @LastEditTime: 2022-06-24
 * @FilePath: /algorithm/515_largest_values/largest_values.rs
 */

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

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

impl Solution {
    pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans = vec![];
        let mut q = VecDeque::new();
        if let Some(node) = root {
            q.push_back(node);
        } else {
            return ans;
        }

        while !q.is_empty() {
            let mut max = i32::MIN;
            for _ in 0..q.len() {
                let node = q.pop_front().unwrap();
                if let Some(left) = node.borrow_mut().left.take() {
                    q.push_back(left);
                }
                if let Some(right) = node.borrow_mut().right.take() {
                    q.push_back(right);
                }
                if node.borrow().val > max {
                    max = node.borrow().val;
                }
            }
            ans.push(max);
        }
        ans
    }
}

fn main() {
    {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 9,
                    left: None,
                    right: None,
                }))),
            }))),
        })));

        let ans = vec![1, 3, 9];

        assert_eq!(Solution::largest_values(root), ans);
    }

    {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            }))),
        })));

        let ans = vec![1, 3];

        assert_eq!(Solution::largest_values(root), ans);
    }
}
