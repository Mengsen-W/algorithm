/*
 * @Date: 2021-04-27 08:38:25
 * @Author: Mengsen Wang
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-02-26
 */

use std::cell::RefCell;
use std::rc::Rc;
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
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        let mut sum = 0;
        let mut stack = vec![root];
        while !stack.is_empty() {
            if let Some(node) = stack.pop() {
                if let Some(node) = node {
                    let left = node.borrow_mut().left.take();
                    if left.is_some() {
                        stack.push(Some(node));
                        stack.push(left);
                    } else {
                        let val = node.borrow().val;
                        if val <= high && val >= low {
                            sum += val;
                        }
                        stack.push(node.borrow_mut().right.take());
                    }
                }
            }
        }
        sum
    }
}

fn range_sum_bst_dfs(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
    if let Some(r) = root {
        let v = r.borrow().val;
        let v = if v >= low && v <= high { v } else { 0 };

        v + range_sum_bst_dfs(r.borrow().left.clone(), low, high)
            + range_sum_bst_dfs(r.borrow().right.clone(), low, high)
    } else {
        0
    }
}

fn range_sum_bst_bfs(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
    let mut sum = 0;
    let mut stack = vec![root];
    while !stack.is_empty() {
        if let Some(node) = stack.pop() {
            if let Some(node) = node {
                let left = node.borrow_mut().left.take();
                if left.is_some() {
                    stack.push(Some(node));
                    stack.push(left);
                } else {
                    let val = node.borrow().val;
                    if val <= high && val >= low {
                        sum += val;
                    }
                    stack.push(node.borrow_mut().right.take());
                }
            }
        }
    }
    sum
}

fn main() {
    let tests = vec![
        (
            Some(Rc::new(RefCell::new(TreeNode {
                val: 10,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 3,
                        left: None,
                        right: None,
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 7,
                        left: None,
                        right: None,
                    }))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 15,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 18,
                        left: None,
                        right: None,
                    }))),
                }))),
            }))),
            7,
            15,
            32,
        ),
        (
            Some(Rc::new(RefCell::new(TreeNode {
                val: 10,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 3,
                        left: Some(Rc::new(RefCell::new(TreeNode {
                            val: 1,
                            left: None,
                            right: None,
                        }))),
                        right: None,
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 7,
                        left: Some(Rc::new(RefCell::new(TreeNode {
                            val: 6,
                            left: None,
                            right: None,
                        }))),
                        right: None,
                    }))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 15,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 18,
                        left: None,
                        right: None,
                    }))),
                }))),
            }))),
            6,
            10,
            23,
        ),
    ];

    for (root, low, high, ans) in tests {
        assert_eq!(Solution::range_sum_bst(root, low, high), ans);
    }
}
