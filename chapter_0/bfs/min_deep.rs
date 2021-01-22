/*
 * @Author: Mengsen.Wang
 * @Date: 2021-01-21 19:56:06
 * @Last Modified by: Mengsen.Wang
 * @Last Modified time: 2021-01-21 20:21:29
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

fn min_depth_dfs(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    match root {
        None => 0,
        Some(mut root) => {
            let (mut depth, mut arr) = (1, vec![root]);
            'found: while !arr.is_empty() {
                let mut children = Vec::new();
                for node in arr {
                    if node.borrow().left.is_none() && node.borrow().right.is_none() {
                        break 'found;
                    }
                    if node.borrow().left.is_some() {
                        children.push(node.borrow().left.clone().unwrap());
                    }
                    if node.borrow().right.is_some() {
                        children.push(node.borrow().right.clone().unwrap());
                    }
                }
                arr = children;
                depth += 1;
            }
            depth
        }
    }
}

fn min_depth_dfs(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if root.is_none() {
        0
    } else {
        let (l, r) = (
            root.as_ref().unwrap().borrow().left.clone(),
            root.as_ref().unwrap().borrow().right.clone(),
        );
        match (l, r) {
            (None, None) => 1,
            (None, nd) | (nd, None) => 1 + min_depth_dfs(nd),
            (l, r) => 1 + std::cmp::min(min_depth_dfs(l), min_depth_dfs(r)),
        }
    }
}

fn main() {}
