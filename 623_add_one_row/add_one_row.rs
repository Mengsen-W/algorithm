/*
 * @Date: 2022-08-05
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-08-05
 * @FilePath: /algorithm/623_add_one_row/add_one_row.rs
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
    pub fn add_one_row(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
        depth: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        use std::collections::VecDeque;
        if depth == 1 {
            let mut node = TreeNode::new(val);
            node.left = root;
            return Some(Rc::new(RefCell::new(node)));
        }
        let mut queue = VecDeque::new();
        queue.push_back(root.clone());
        for _ in 0..depth - 2 {
            let size = queue.len();
            for _ in 0..size {
                if let Some(node) = queue.pop_front() {
                    if node.as_ref().unwrap().borrow().left.is_some() {
                        queue.push_back(node.as_ref().unwrap().borrow().left.clone());
                    }
                    if node.as_ref().unwrap().borrow().right.is_some() {
                        queue.push_back(node.as_ref().unwrap().borrow().right.clone());
                    }
                }
            }
        }
        while !queue.is_empty() {
            if let Some(node) = queue.pop_front() {
                let mut new_node = TreeNode::new(val);
                new_node.left = node.as_ref().unwrap().borrow().left.clone();
                node.as_ref().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(new_node)));
                let mut new_node = TreeNode::new(val);
                new_node.right = node.as_ref().unwrap().borrow().right.clone();
                node.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(new_node)));
            }
        }
        root
    }
}

fn main() {
    {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 6,
                left: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
                right: None,
            }))),
        })));

        let val = 1;
        let depth = 2;

        let ans = Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                }))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 6,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
                    right: None,
                }))),
            }))),
        })));
        assert_eq!(Solution::add_one_row(root, val, depth), ans);
    }

    {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            }))),
            right: None,
        })));

        let val = 1;
        let depth = 3;

        let ans = Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                }))),
            }))),
            right: None,
        })));
        assert_eq!(Solution::add_one_row(root, val, depth), ans);
    }
}
