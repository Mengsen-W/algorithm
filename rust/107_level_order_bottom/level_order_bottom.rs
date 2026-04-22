/*
 * @Date: 2024-02-15
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-02-15
 * @FilePath: /algorithm/rust/107_level_order_bottom/level_order_bottom.rs
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
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut ans = Vec::new();
        let mut q = VecDeque::new();
        if let Some(x) = root {
            q.push_back(x);
        }
        while !q.is_empty() {
            let n = q.len();
            let mut vals = Vec::with_capacity(n); // 预分配空间
            for _ in 0..n {
                if let Some(node) = q.pop_front() {
                    let mut x = node.borrow_mut();
                    vals.push(x.val);
                    if let Some(left) = x.left.take() {
                        q.push_back(left);
                    }
                    if let Some(right) = x.right.take() {
                        q.push_back(right);
                    }
                }
            }
            ans.push(vals);
        }
        ans.reverse();
        ans
    }
}

fn main() {
    let tests = vec![
        (
            Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 20,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(15)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
                }))),
            }))),
            vec![vec![15, 7], vec![9, 20], vec![3]],
        ),
        (Some(Rc::new(RefCell::new(TreeNode::new(1)))), vec![vec![1]]),
        (None, vec![]),
    ];

    for (root, ans) in tests {
        assert_eq!(Solution::level_order_bottom(root), ans);
    }
}
