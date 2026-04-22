/*
 * @Date: 2023-05-20
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-05-20
 * @FilePath: /algorithm/rust/1373_max_sum_bst/max_sum_bst.rs
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
    fn DFS(root: Option<Rc<RefCell<TreeNode>>>, result: &mut i32) -> (bool, i32, i32, i32) {
        if root.is_none() {
            return (true, 2147483647, -2147483648, 0);
        }
        let root = root.unwrap();
        let root_node = root.borrow();
        let left_result = Self::DFS(root_node.left.clone(), result);
        let right_result = Self::DFS(root_node.right.clone(), result);

        let root_node_val = root_node.val;

        if left_result.0
            && right_result.0
            && root_node_val > left_result.2
            && root_node_val < right_result.1
        {
            let sum = left_result.3 + right_result.3 + root_node_val;
            *result = std::cmp::max(*result, sum);
            (
                true,
                std::cmp::min(left_result.1, root_node_val),
                std::cmp::max(root_node_val, right_result.2),
                sum,
            )
        } else {
            (false, 0, 0, -21474836)
        }
    }
    pub fn max_sum_bst(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut result: i32 = 0;
        let _ = Self::DFS(root, &mut result);
        result
    }
}

fn main() {
    {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
                }))),
            }))),
        })));
        let ans = 20;
        assert_eq!(Solution::max_sum_bst(root), ans);
    }

    {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
            }))),
            right: None,
        })));
        let ans = 2;
        assert_eq!(Solution::max_sum_bst(root), ans);
    }

    {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode::new(-2)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(-5)))),
        })));
        let ans = 0;
        assert_eq!(Solution::max_sum_bst(root), ans);
    }

    {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        })));
        let ans = 6;
        assert_eq!(Solution::max_sum_bst(root), ans);
    }

    {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 8,
                left: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            }))),
        })));
        let ans = 7;
        assert_eq!(Solution::max_sum_bst(root), ans);
    }
}
