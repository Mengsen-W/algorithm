/*
 * @Date: 2022-07-31
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-07-31
 * @FilePath: /algorithm/1161_max_level_sum/max_level_sum.rs
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
    pub fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(r) = root {
            let mut stack = vec![];
            let mut tmp = vec![];
            let mut ans = 0;
            let mut depth = 0;
            let mut sum = i32::MIN;
            stack.push(r);

            while stack.len() > 0 {
                let n = stack.len();
                depth += 1;
                let mut cur = 0;

                for i in 0..n {
                    cur += stack[i].borrow().val;

                    if let Some(left) = stack[i].borrow_mut().left.take() {
                        tmp.push(left);
                    }

                    if let Some(right) = stack[i].borrow_mut().right.take() {
                        tmp.push(right);
                    }
                }

                if sum < cur {
                    sum = cur;
                    ans = depth;
                }

                stack = tmp;
                tmp = vec![];
            }

            ans
        } else {
            0
        }
    }
}

fn main() {
    {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 7,
                left: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(-8)))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
        })));
        assert_eq!(Solution::max_level_sum(root), 2);
    }

    {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 989,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 10250,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 98693,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: -89388,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode::new(-32127)))),
                }))),
            }))),
        })));
        assert_eq!(Solution::max_level_sum(root), 2);
    }
}
