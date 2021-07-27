/*
 * @Date: 2021-07-27 17:38:07
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-07-27 18:50:42
 */

// Definition for a binary tree node.
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
    pub fn new(
        val: i32,
        left: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>>,
    ) -> Self {
        TreeNode { val, left, right }
    }
}

fn find_second_minimum_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn pre_order(node: Option<Rc<RefCell<TreeNode>>>, vec: &mut Vec<i32>) {
        if let Some(node) = node {
            let v = node.borrow().val;
            vec.push(v);
            pre_order(node.borrow_mut().left.take(), vec);
            pre_order(node.borrow_mut().right.take(), vec);
        }
    }
    let mut vec = Vec::new();
    pre_order(root, &mut vec);
    vec.sort();
    vec.dedup();
    if vec.len() < 2 {
        return -1;
    }
    vec[1]
}

fn main() {
    {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 5,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 7,
                    left: None,
                    right: None,
                }))),
            }))),
        })));
        assert_eq!(find_second_minimum_value(root), 5);
    }
    {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: None,
            }))),
        })));
        assert_eq!(find_second_minimum_value(root), -1);
    }
}
