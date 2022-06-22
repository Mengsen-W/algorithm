/*
 * @Author: Mengsen Wang mengsen_wang@163.com
 * @Date: 2022-06-22
 * @LastEditors: Mengsen Wang mengsen_wang@163.com
 * @LastEditTime: 2022-06-22
 * @FilePath: /algorithm/513_find_bottom_left_value/find_bottom_left_value.rs
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
    pub fn find_bottom_left_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // there must be a node
        let node = root.unwrap();
        let mut queue = vec![node];
        let mut res = 0;

        while queue.len() > 0 {
            let mut new_queue = vec![];

            queue.iter().enumerate().for_each(|(i, node)| {
                if i == 0 {
                    res = node.borrow().val;
                }

                if let Some(left) = node.borrow().left.clone() {
                    new_queue.push(left);
                }
                if let Some(right) = node.borrow().right.clone() {
                    new_queue.push(right);
                }
            });

            queue = new_queue;
        }

        res
    }
}

fn main() {
    {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        })));
        assert_eq!(Solution::find_bottom_left_value(root), 1);
    }

    {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
            }))),
        })));
        assert_eq!(Solution::find_bottom_left_value(root), 7);
    }
}
