/*
 * @Date: 2021-09-28 09:55:34
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-09-28 11:31:27
 */

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
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

impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        let mut prefix: HashMap<i32, i32> = HashMap::new();
        prefix.insert(0, 1);
        Self::dfs(&root, 0, target_sum, &mut prefix)
    }
    fn dfs(
        root: &Option<Rc<RefCell<TreeNode>>>,
        mut curr: i32,
        target_sum: i32,
        prefix: &mut HashMap<i32, i32>,
    ) -> i32 {
        if root.is_none() {
            return 0;
        }
        let mut ret = 0;
        curr += root.as_ref().unwrap().borrow().val;
        if prefix.contains_key(&(curr - target_sum)) {
            ret = *prefix.get(&(curr - target_sum)).unwrap();
        }
        *prefix.entry(curr).or_insert(0) += 1;
        ret += Self::dfs(
            &root.as_ref().unwrap().borrow().left,
            curr,
            target_sum,
            prefix,
        );
        ret += Self::dfs(
            &root.as_ref().unwrap().borrow().right,
            curr,
            target_sum,
            prefix,
        );
        *prefix.get_mut(&curr).unwrap() -= 1;
        ret
    }
}

fn main() {
    {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 10,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 5,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(-2)))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode::new(-2)))),
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: -3,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode::new(11)))),
            }))),
        })));
        let target_sum = 8;
        assert_eq!(Solution::path_sum(root, target_sum), 3);
    }
    {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 11,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
                }))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 8,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 13,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                }))),
            }))),
        })));
        let target_sum = 22;
        assert_eq!(Solution::path_sum(root, target_sum), 3);
    }
}
