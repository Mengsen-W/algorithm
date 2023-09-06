/*
 * @Date: 2023-09-06
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-09-06
 * @FilePath: /algorithm/rust/1123_lca_deepest_leaves/lca_deepest_leaves.rs
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
    pub fn lca_deepest_leaves(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        dfs(root).1
    }
}

type Tree = Rc<RefCell<TreeNode>>;

fn dfs(root: Option<Tree>) -> (u8, Option<Tree>) {
    match root {
        None => (0, None),
        Some(tree) => {
            let node = tree.borrow();
            let (l_depth, l_res) = dfs(node.left.clone());
            let (r_depth, r_res) = dfs(node.right.clone());
            if l_depth == r_depth {
                (l_depth + 1, Some(tree.clone()))
            } else if l_depth > r_depth {
                (l_depth + 1, l_res)
            } else {
                (r_depth + 1, r_res)
            }
        }
    }
}

fn main() {
    let tests = vec![
        (
            Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 2,
                        left: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
                        right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                    }))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(8)))),
                }))),
            }))),
            Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
            }))),
        ),
        (
            Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            Some(Rc::new(RefCell::new(TreeNode::new(1)))),
        ),
        (
            Some(Rc::new(RefCell::new(TreeNode {
                val: 0,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            }))),
            Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        ),
    ];

    for (root, ans) in tests {
        assert_eq!(Solution::lca_deepest_leaves(root), ans);
    }
}
