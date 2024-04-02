/*
 * @Date: 2024-04-02
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-04-02
 * @FilePath: /algorithm/rust/894_all_possible_fbt/all_possible_fbt.rs
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

impl Solution {
    pub fn all_possible_fbt(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if n % 2 == 0 {
            return vec![];
        }
        let mut dp: Vec<Vec<Option<Rc<RefCell<TreeNode>>>>> = vec![Vec::new(); (n + 1) as usize];
        dp[1].push(Some(Rc::new(RefCell::new(TreeNode::new(0)))));
        for i in (3..=n).step_by(2) {
            let mut full_binary_trees: Vec<Option<Rc<RefCell<TreeNode>>>> = Vec::new();
            for j in (1..i).step_by(2) {
                for left_subtree in &dp[j as usize] {
                    for right_subtree in &dp[(i - 1 - j) as usize] {
                        let mut root = Rc::new(RefCell::new(TreeNode {
                            val: 0,
                            left: left_subtree.clone(),
                            right: right_subtree.clone(),
                        }));
                        full_binary_trees.push(Some(root.clone()));
                    }
                }
            }
            dp[i as usize] = full_binary_trees.clone();
        }
        dp[n as usize].clone()
    }
}
