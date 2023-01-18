/*
 * @Date: 2022-06-02 09:53:47
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-06-02 11:35:51
 * @FilePath: /algorithm/450_delete_bst_node/delete_bst_node.rs
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
use std::rc::Rc;

struct Solution;

impl Solution {
    pub fn delete_node(
        root: Option<Rc<RefCell<TreeNode>>>,
        key: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return None;
        }
        let root_node = root.unwrap();

        if root_node.borrow().val > key {
            let t = Solution::delete_node(root_node.borrow().left.clone(), key);
            root_node.borrow_mut().left = t;
        } else if root_node.borrow().val < key {
            let t = Solution::delete_node(root_node.borrow().right.clone(), key);
            root_node.borrow_mut().right = t;
        } else {
            if root_node.borrow().left.is_none() {
                return root_node.borrow().right.clone();
            }
            if root_node.borrow().right.is_none() {
                return root_node.borrow().left.clone();
            }
            let right_min = Solution::find_right_min(root_node.borrow().right.clone());
            root_node.borrow_mut().val = right_min.unwrap().borrow().val;
            let t = Solution::delete_node(root_node.borrow().right.clone(), root_node.borrow().val);
            root_node.borrow_mut().right = t;
        }

        Some(root_node)
    }

    fn find_right_min(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return None;
        }

        let mut root_node = root.unwrap();
        while root_node.borrow().left.is_some() {
            let t = root_node.borrow().left.clone();
            root_node = t.unwrap();
        }

        Some(root_node)
    }
}

fn main() {
    assert_eq!(Solution::delete_node(None, 1), None);
    assert_eq!(
        Solution::delete_node(
            Some(Rc::new(RefCell::new(TreeNode {
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
            }))),
            3
        ),
        Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: None,
                }))),
                right: None,
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
        }))),
    );

    assert_eq!(
        Solution::delete_node(
            Some(Rc::new(RefCell::new(TreeNode {
                val: 5,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
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
            }))),
            0
        ),
        Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
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
        }))),
    );
}
