/*
 * @Date: 2022-03-21 00:26:33
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-03-21 04:30:30
 * @FilePath: /algorithm/653_find_target/find_target.rs
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
use std::collections::{HashSet, VecDeque};
use std::rc::Rc;

struct Solution;

impl Solution {
    pub fn find_target(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
        let mut set = HashSet::new();
        let mut q = VecDeque::new();
        q.push_back(root);
        while let Some(node) = q.pop_front() {
            if let Some(node) = node {
                let mut node = node.as_ref().borrow_mut();
                if set.contains(&node.val) {
                    return true;
                }
                set.insert(k - node.val);
                q.push_back(node.left.take());
                q.push_back(node.right.take());
            }
        }
        false
    }
}

fn main() {
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 5,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: None,
                right: None,
            }))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 6,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 7,
                left: None,
                right: None,
            }))),
        }))),
    })));

    assert_eq!(
        Solution::find_target(root.clone(), 9),
        true,
        "find_target(root, 9) should return true"
    );
    assert_eq!(
        Solution::find_target(root, 9),
        false,
        "find_target(root, 28) should return false"
    );
}
