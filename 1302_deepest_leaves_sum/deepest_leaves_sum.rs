/*
 * @Date: 2022-08-17
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-08-17
 * @FilePath: /algorithm/1302_deepest_leaves_sum/deepest_leaves_sum.rs
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
    pub fn deepest_leaves_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        use std::collections::VecDeque;
        let (mut queue, mut ret) = (VecDeque::new(), 0);
        queue.push_back(root);

        while !queue.is_empty() {
            ret = 0;
            for _ in 0..queue.len() {
                let node = queue.pop_front().unwrap();
                ret += node.as_ref().unwrap().borrow().val;
                if node.as_ref().unwrap().borrow().left.is_some() {
                    queue.push_back(node.as_ref().unwrap().borrow().left.clone());
                }
                if node.as_ref().unwrap().borrow().right.is_some() {
                    queue.push_back(node.as_ref().unwrap().borrow().right.clone());
                }
            }
        }
        ret
    }
}

fn main() {
    {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 6,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode::new(8)))),
                }))),
            }))),
        })));
        assert_eq!(Solution::deepest_leaves_sum(root), 15);
    }
    {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 6,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 7,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 7,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 8,
                left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
                }))),
            }))),
        })));
        assert_eq!(Solution::deepest_leaves_sum(root), 19);
    }
}
