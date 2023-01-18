/*
 * @Date: 2021-10-17 09:40:52
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-10-17 10:13:41
 */
use std::cell::RefCell;
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
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut k = k;
        let mut r = root;
        let mut stack = Vec::with_capacity(1024);
        while r.is_some() || !stack.is_empty() {
            while let Some(t) = r {
                stack.push(t.clone());
                r = t.clone().borrow_mut().left.take();
            }
            if let Some(t) = stack.pop() {
                k -= 1;
                if k == 0 {
                    return t.borrow().val;
                }
                r = t.clone().borrow_mut().right.take();
            }
        }
        -1
    }
}

fn main() {
    {
        assert_eq!(
            Solution::kth_smallest(
                Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 1,
                        left: None,
                        right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                }))),
                1
            ),
            1
        );
    }

    {
        assert_eq!(
            Solution::kth_smallest(
                Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 3,
                        left: Some(Rc::new(RefCell::new(TreeNode {
                            val: 2,
                            left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                            right: None,
                        }))),
                        right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
                }))),
                3
            ),
            3
        );
    }
}
