/*
 * @Date: 2022-06-19 16:53:38
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-06-19 17:32:50
 * @FilePath: /algorithm/508_find_frequent_tree_sum/find_frequent_tree_sum.rs
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
    pub fn find_frequent_tree_sum(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut map = std::collections::HashMap::new();
        Self::dfs(&root, &mut map);
        let max = map.iter().max_by_key(|(_, v)| **v).unwrap().1;
        map.iter()
            .filter(|(_, v)| *v == max)
            .map(|(k, _)| k)
            .copied()
            .collect()
    }

    fn dfs(
        root: &Option<Rc<RefCell<TreeNode>>>,
        map: &mut std::collections::HashMap<i32, i32>,
    ) -> i32 {
        if let Some(node) = root {
            let sum = node.borrow().val
                + Self::dfs(&node.borrow().left, map)
                + Self::dfs(&node.borrow().right, map);
            *map.entry(sum).or_insert(0) += 1;
            sum
        } else {
            0
        }
    }
}

fn main() {
    {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: -3,
                left: None,
                right: None,
            }))),
        })));
        assert_eq!(Solution::find_frequent_tree_sum(root), vec![2, 4, -3]);
    }
    {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: -5,
                left: None,
                right: None,
            }))),
        })));
        assert_eq!(Solution::find_frequent_tree_sum(root), vec![2]);
    }
}
