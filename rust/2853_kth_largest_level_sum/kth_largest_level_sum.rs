/*
 * @Date: 2024-02-23
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-02-23
 * @FilePath: /algorithm/rust/2853_kth_largest_level_sum/kth_largest_level_sum.rs
 */
struct Solution;

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

impl Solution {
    pub fn kth_largest_level_sum(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i64 {
        let mut a = Vec::new();
        let mut cur = vec![root.unwrap()];
        while !cur.is_empty() {
            let mut sum = 0i64;
            let mut nxt = Vec::new();
            for node in cur {
                let mut x = node.borrow_mut();
                sum += x.val as i64;
                if let Some(left) = x.left.take() {
                    nxt.push(left);
                }
                if let Some(right) = x.right.take() {
                    nxt.push(right);
                }
            }
            cur = nxt;
            a.push(sum);
        }
        let k = k as usize;
        if k > a.len() {
            return -1;
        }
        a.sort_unstable();
        a[a.len() - k]
    }
}

fn main() {
    let tests = vec![
        (
            Some(Rc::new(RefCell::new(TreeNode {
                val: 5,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 8,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 2,
                        left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                        right: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 9,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
                }))),
            }))),
            2,
            13,
        ),
        (
            Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                    right: None,
                }))),
                right: None,
            }))),
            1,
            3,
        ),
    ];

    for (root, k, ans) in tests {
        assert_eq!(Solution::kth_largest_level_sum(root, k), ans);
    }
}
