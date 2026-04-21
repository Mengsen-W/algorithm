/*
 * @Date: 2023-11-25
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-11-25
 * @FilePath: /algorithm/rust/1457_pseudo_palindromic_paths/pseudo_palindromic_paths.rs
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
    pub fn pseudo_palindromic_paths(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(node: Option<&Rc<RefCell<TreeNode>>>, mut mask: i32) -> i32 {
            if let Some(x) = node {
                let x = x.borrow();
                mask ^= 1 << x.val; // 修改 x.val 出现次数的奇偶性
                if x.left.is_none() && x.right.is_none() {
                    // x 是叶子节点
                    return if (mask & (mask - 1)) == 0 { 1 } else { 0 };
                }
                return dfs(x.left.as_ref(), mask) + dfs(x.right.as_ref(), mask);
            }
            0
        }
        dfs(root.as_ref(), 0)
    }
}

fn main() {
    let tests = vec![
        (
            Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                    right: None,
                }))),
            }))),
            2,
        ),
        (
            Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 3,
                        left: None,
                        right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                    }))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            }))),
            1,
        ),
        (Some(Rc::new(RefCell::new(TreeNode::new(1)))), 1),
    ];

    for (root, ans) in tests {
        assert_eq!(Solution::pseudo_palindromic_paths(root), ans);
    }
}
