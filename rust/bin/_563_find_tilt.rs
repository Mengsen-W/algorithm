/*
 * @Date: 2021-11-18 00:18:22
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-11-18 02:22:18
 */

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
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

struct Solution;

impl Solution {
    pub fn find_tilt(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(node: Option<Rc<RefCell<TreeNode>>>, ret: &mut i32) -> i32 {
            if let Some(node) = node {
                let v = node.borrow().val;
                let lsum = dfs(node.borrow_mut().left.take(), ret);
                let rsum = dfs(node.borrow_mut().right.take(), ret);
                *ret += (lsum - rsum).abs();
                return lsum + rsum + v;
            } else {
                return 0;
            }
        }
        let mut ret = 0;
        dfs(root, &mut ret);
        ret
    }
}

fn main() {
    assert_eq!(
        1,
        Solution::find_tilt(Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: (Some(Rc::new(RefCell::new(TreeNode::new(2))))),
            right: (Some(Rc::new(RefCell::new(TreeNode::new(3))))),
        }))))
    );
    assert_eq!(
        15,
        Solution::find_tilt(Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: (Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: (Some(Rc::new(RefCell::new(TreeNode::new(3))))),
                right: (Some(Rc::new(RefCell::new(TreeNode::new(4))))),
            })))),
            right: (Some(Rc::new(RefCell::new(TreeNode {
                val: 9,
                left: None,
                right: (Some(Rc::new(RefCell::new(TreeNode::new(7))))),
            })))),
        }))))
    );
    assert_eq!(
        9,
        Solution::find_tilt(Some(Rc::new(RefCell::new(TreeNode {
            val: 21,
            left: (Some(Rc::new(RefCell::new(TreeNode {
                val: 7,
                left: (Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: (Some(Rc::new(RefCell::new(TreeNode::new(3))))),
                    right: (Some(Rc::new(RefCell::new(TreeNode::new(3))))),
                })))),
                right: (Some(Rc::new(RefCell::new(TreeNode::new(1))))),
            })))),
            right: (Some(Rc::new(RefCell::new(TreeNode {
                val: 14,
                left: (Some(Rc::new(RefCell::new(TreeNode::new(2))))),
                right: (Some(Rc::new(RefCell::new(TreeNode::new(2))))),
            })))),
        }))))
    );
}
