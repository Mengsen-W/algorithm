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
    pub fn subtree_with_all_deepest(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Self::dfs(&root).1
    }

    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>) -> (i32, Option<Rc<RefCell<TreeNode>>>) {
        match node {
            None => (0, None),
            Some(node_ref) => {
                let node_borrow = node_ref.borrow();
                let (left_depth, left_lca) = Self::dfs(&node_borrow.left);
                let (right_depth, right_lca) = Self::dfs(&node_borrow.right);

                if left_depth > right_depth {
                    (left_depth + 1, left_lca)
                } else if left_depth < right_depth {
                    (right_depth + 1, right_lca)
                } else {
                    (left_depth + 1, Some(node_ref.clone()))
                }
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
        assert_eq!(Solution::subtree_with_all_deepest(root), ans);
    }
}
