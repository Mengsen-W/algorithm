/*
 * @Date: 2022-08-27
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-08-27
 * @FilePath: /algorithm/662_width_of_binary_tree/width_of_binary_tree.rs
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
    pub fn width_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut ans = 0;
        let mut queue = vec![];

        if let Some(r) = root {
            queue.push((r, 1));

            while queue.len() > 0 {
                let mut tmp = vec![];
                let start = queue[0].1;
                let mut index = 0;

                for i in 0..queue.len() {
                    index = queue[i].1;

                    if let Some(left) = queue[i].0.borrow_mut().left.take() {
                        tmp.push((left, index * 2 - start * 2));
                    }

                    if let Some(right) = queue[i].0.borrow_mut().right.take() {
                        tmp.push((right, index * 2 + 1 - start * 2));
                    }
                }

                ans = ans.max(index - start + 1);
                queue = tmp;
            }
        }

        ans
    }
}

fn main() {
    {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
            }))),
        })));
        let ans = 4;
        assert_eq!(Solution::width_of_binary_tree(root), ans);
    }

    {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
                    right: None,
                }))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 9,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
                    right: None,
                }))),
            }))),
        })));
        let ans = 7;
        assert_eq!(Solution::width_of_binary_tree(root), ans);
    }

    {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        })));
        let ans = 2;
        assert_eq!(Solution::width_of_binary_tree(root), ans);
    }
}
