/*
 * @Date: 2023-12-15
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-12-15
 * @FilePath: /algorithm/rust/2415_reverse_odd_levels/reverse_odd_levels.rs
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
    pub fn reverse_odd_levels(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut layer = vec![root.clone()];
        let mut count = 0;
        while !layer.is_empty() {
            if count % 2 != 0 {
                let (mut l, mut r) = (0, layer.len() - 1);
                while l < r {
                    let f = layer[l].as_ref().unwrap().try_borrow_mut();
                    let b = layer[r].as_ref().unwrap().try_borrow_mut();
                    std::mem::swap(&mut f.unwrap().val, &mut b.unwrap().val);
                    l += 1;
                    r -= 1;
                }
            }
            let mut next = vec![];
            while let Some(p) = layer.pop() {
                let n = p.unwrap();
                if n.borrow().left.is_some() {
                    next.push(n.borrow().left.clone());
                }
                if n.borrow().right.is_some() {
                    next.push(n.borrow().right.clone());
                }
            }
            layer = next;
            count += 1;
        }

        root
    }
}

fn main() {
    let tests = vec![
        (
            Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(8)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(13)))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(21)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(34)))),
                }))),
            }))),
            Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(8)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(13)))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(21)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(34)))),
                }))),
            }))),
        ),
        (
            Some(Rc::new(RefCell::new(TreeNode {
                val: 7,
                left: Some(Rc::new(RefCell::new(TreeNode::new(11)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(13)))),
            }))),
            Some(Rc::new(RefCell::new(TreeNode {
                val: 7,
                left: Some(Rc::new(RefCell::new(TreeNode::new(13)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(11)))),
            }))),
        ),
        (
            Some(Rc::new(RefCell::new(TreeNode {
                val: 0,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 0,
                        left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                        right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 0,
                        left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                        right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                    }))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 0,
                        left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
                        right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 0,
                        left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
                        right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
                    }))),
                }))),
            }))),
            Some(Rc::new(RefCell::new(TreeNode {
                val: 0,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 0,
                        left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
                        right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 0,
                        left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
                        right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
                    }))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 0,
                        left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                        right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 0,
                        left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                        right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                    }))),
                }))),
            }))),
        ),
    ];

    for (test, ans) in tests {
        assert_eq!(Solution::reverse_odd_levels(test), ans);
    }
}
