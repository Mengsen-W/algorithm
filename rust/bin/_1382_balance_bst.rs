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
    pub fn balance_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut inorder_seq = Vec::new();

        Self::get_inorder(&root, &mut inorder_seq);
        Self::build(&inorder_seq, 0, inorder_seq.len() as i32 - 1)
    }

    fn get_inorder(root: &Option<Rc<RefCell<TreeNode>>>, seq: &mut Vec<i32>) {
        if let Some(node) = root {
            let node_ref = node.borrow();
            Self::get_inorder(&node_ref.left, seq);
            seq.push(node_ref.val);
            Self::get_inorder(&node_ref.right, seq);
        }
    }

    fn build(seq: &[i32], l: i32, r: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if l > r {
            return None;
        }
        let mid = (l + r) >> 1;
        let mut node = TreeNode::new(seq[mid as usize]);
        node.left = Self::build(seq, l, mid - 1);
        node.right = Self::build(seq, mid + 1, r);
        Some(Rc::new(RefCell::new(node)))
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
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 3,
                        left: None,
                        right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                    }))),
                }))),
            }))),
            Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                }))),
            }))),
        ),
        (
            Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            }))),
            Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            }))),
        ),
    ];

    for (root, expected) in tests {
        assert_eq!(Solution::balance_bst(root), expected);
    }
}
