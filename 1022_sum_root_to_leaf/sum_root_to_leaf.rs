/*
 * @Date: 2022-05-31 09:44:21
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-05-31 10:00:47
 * @FilePath: /algorithm/1022_sum_root_to_leaf/sum_root_to_leaf.rs
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
    pub fn sum_root_to_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut result = 0;
        let mut data = 0;
        let mut current_level = -1;

        let mut stack = vec![];

        if let None = root {
            return 0;
        }

        stack.push((root.unwrap(), 0));

        while stack.len() > 0 {
            let (node, level) = stack.pop().unwrap();
            let node = node.borrow();
            data = data >> (current_level - level + 1);
            data = (data << 1) + node.val;

            current_level = level;

            if node.left.is_some() {
                stack.push((node.left.clone().unwrap(), level + 1));
            }
            if node.right.is_some() {
                stack.push((node.right.clone().unwrap(), level + 1));
            }

            if node.left.is_none() && node.right.is_none() {
                result += data;
            }
        }

        result
    }
}

fn main() {
    assert_eq!(
        Solution::sum_root_to_leaf(Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 0,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 0,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: None,
                    right: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 0,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: None,
                    right: None,
                }))),
            }))),
        })))),
        22
    );

    assert_eq!(
        Solution::sum_root_to_leaf(Some(Rc::new(RefCell::new(TreeNode {
            val: 0,
            left: None,
            right: None,
        }))),),
        0
    );
}
