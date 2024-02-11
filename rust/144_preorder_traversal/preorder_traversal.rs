/*
 * @Date: 2024-02-11
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-02-11
 * @FilePath: /algorithm/rust/144_preorder_traversal/preorder_traversal.rs
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
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = Vec::new();
        Self::dfs(root.clone(), &mut res);
        res
    }
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>) {
        if let Some(node) = root {
            res.push(node.borrow().val);
            Self::dfs(node.borrow().left.clone(), res);
            Self::dfs(node.borrow().right.clone(), res);
        }
    }
}

fn main() {
    let tests = vec![
        (
            Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                    right: None,
                }))),
            }))),
            vec![1, 2, 3],
        ),
        (None, vec![]),
        (
            Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
            }))),
            vec![1, 2],
        ),
        (
            Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
                right: None,
            }))),
            vec![1, 2],
        ),
    ];

    for (root, ans) in tests {
        assert_eq!(Solution::preorder_traversal(root), ans);
    }
}
