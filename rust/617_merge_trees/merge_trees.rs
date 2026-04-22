/*
 * @Date: 2023-08-14
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-08-14
 * @FilePath: /algorithm/rust/617_merge_trees/merge_trees.rs
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
    pub fn merge_trees(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match (root1.clone(), root2.clone()) {
            (Some(r1), Some(r2)) => {
                let r1_child_left = r1.borrow_mut().left.take();
                let r1_child_right = r1.borrow_mut().right.take();

                let r2_child_left = r2.borrow_mut().left.take();
                let r2_child_right = r2.borrow_mut().right.take();

                Some(Rc::new(RefCell::new(TreeNode {
                    val: r1.borrow().val + r2.borrow().val,
                    left: Self::merge_trees(r1_child_left, r2_child_left),
                    right: Self::merge_trees(r1_child_right, r2_child_right),
                })))
            }
            _ => root1.or(root2),
        }
    }
}

pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut ans = vec![];
    let mut stack = vec![root];
    while let Some(node) = stack.pop() {
        if let Some(n) = node {
            ans.push(n.borrow().val);
            stack.push(n.borrow_mut().right.take());
            stack.push(n.borrow_mut().left.take());
        }
    }
    ans
}

fn main() {
    let tests = vec![
        (
            Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
            }))),
            Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
                }))),
            }))),
            Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
                }))),
            }))),
        ),
        (
            Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
                right: None,
            }))),
            Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
                right: None,
            }))),
        ),
    ];

    for (t1, t2, ans) in tests {
        assert_eq!(
            preorder_traversal(Solution::merge_trees(t1, t2)),
            preorder_traversal(ans)
        );
    }
}
