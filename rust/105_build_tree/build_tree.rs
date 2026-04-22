/*
 * @Date: 2024-02-20
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-02-20
 * @FilePath: /algorithm/rust/105_build_tree/build_tree.rs
 */

struct Solution;

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
use std::collections::HashMap;
use std::rc::Rc;

impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let n = preorder.len();
        let mut index = HashMap::with_capacity(n);
        for (i, &x) in inorder.iter().enumerate() {
            index.insert(x, i);
        }

        fn dfs(
            preorder: &Vec<i32>,
            pre_l: usize,
            pre_r: usize,
            inorder: &Vec<i32>,
            in_l: usize,
            in_r: usize,
            index: &HashMap<i32, usize>,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            if pre_l == pre_r {
                return None;
            }
            let left_size = index[&preorder[pre_l]] - in_l;
            let left = dfs(
                preorder,
                pre_l + 1,
                pre_l + 1 + left_size,
                inorder,
                in_l,
                in_l + left_size,
                index,
            );
            let right = dfs(
                preorder,
                pre_l + 1 + left_size,
                pre_r,
                inorder,
                in_l + 1 + left_size,
                in_r,
                index,
            );
            Some(Rc::new(RefCell::new(TreeNode {
                val: preorder[pre_l],
                left,
                right,
            })))
        }
        dfs(&preorder, 0, n, &inorder, 0, n, &index)
    }
}

fn main() {
    let tests = vec![
        (
            vec![3, 9, 20, 15, 7],
            vec![9, 3, 15, 20, 7],
            Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 20,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(15)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
                }))),
            }))),
        ),
        (
            vec![-1],
            vec![-1],
            Some(Rc::new(RefCell::new(TreeNode::new(-1)))),
        ),
    ];

    for (preorder, inorder, ans) in tests {
        assert_eq!(Solution::build_tree(preorder, inorder), ans,);
    }
}
