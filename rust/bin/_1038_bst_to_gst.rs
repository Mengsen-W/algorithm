/*
 * @Date: 2023-12-04
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-12-04
 * @FilePath: /algorithm/rust/1038_bst_to_gst/bst_to_gst.rs
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
    pub fn bst_to_gst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        fn dfs(node: Option<&Rc<RefCell<TreeNode>>>, s: &mut i32) {
            if let Some(x) = node {
                let mut x = x.borrow_mut();
                dfs(x.right.as_ref(), s); // 递归右子树
                *s += x.val;
                x.val = *s; // 此时 s 就是 >= node.val 的所有数之和
                dfs(x.left.as_ref(), s); // 递归左子树
            }
        }
        let mut s = 0;
        dfs(root.as_ref(), &mut s);
        root
    }
}

fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
    match (p, q) {
        (Some(p), Some(q)) => {
            let pb = p.borrow();
            let qb = q.borrow();
            pb.val == qb.val
                && is_same_tree(pb.left.clone(), qb.left.clone())
                && is_same_tree(pb.right.clone(), qb.right.clone())
        }
        (None, None) => true,
        _ => false,
    }
}

fn main() {
    let tests = vec![
        (
            Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 2,
                        left: None,
                        right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                    }))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 6,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 7,
                        left: None,
                        right: Some(Rc::new(RefCell::new(TreeNode::new(8)))),
                    }))),
                }))),
            }))),
            Some(Rc::new(RefCell::new(TreeNode {
                val: 30,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 36,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(36)))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 35,
                        left: None,
                        right: Some(Rc::new(RefCell::new(TreeNode::new(33)))),
                    }))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 21,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(26)))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 15,
                        left: None,
                        right: Some(Rc::new(RefCell::new(TreeNode::new(8)))),
                    }))),
                }))),
            }))),
        ),
        (
            Some(Rc::new(RefCell::new(TreeNode {
                val: 0,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            }))),
            Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            }))),
        ),
    ];

    for (root, ans) in tests {
        assert_eq!(is_same_tree(Solution::bst_to_gst(root), ans), true);
    }
}
