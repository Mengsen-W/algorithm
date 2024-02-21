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
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let n = inorder.len();
        let mut index = HashMap::with_capacity(n);
        for (i, &x) in inorder.iter().enumerate() {
            index.insert(x, i);
        }

        fn dfs(
            inorder: &Vec<i32>,
            in_l: usize,
            in_r: usize,
            postorder: &Vec<i32>,
            post_l: usize,
            post_r: usize,
            index: &HashMap<i32, usize>,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            if post_l == post_r {
                // 空节点
                return None;
            }
            let val = postorder[post_r - 1];
            let left_size = index[&val] - in_l;
            let left = dfs(
                inorder,
                in_l,
                in_l + left_size,
                postorder,
                post_l,
                post_l + left_size,
                index,
            );
            let right = dfs(
                inorder,
                in_l + left_size + 1,
                in_r,
                postorder,
                post_l + left_size,
                post_r - 1,
                index,
            );
            Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
        }
        dfs(&inorder, 0, n, &postorder, 0, n, &index) // 左闭右开区间
    }
}

fn main() {
    let tests = vec![
        (
            vec![9, 3, 15, 20, 7],
            vec![9, 15, 7, 20, 3],
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
