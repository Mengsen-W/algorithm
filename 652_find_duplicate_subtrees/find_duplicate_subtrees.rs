/*
 * @Date: 2022-09-05
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-09-05
 * @FilePath: /algorithm/652_find_duplicate_subtrees/find_duplicate_subtrees.rs
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
use std::collections::HashMap;
use std::rc::Rc;
impl Solution {
    pub fn find_duplicate_subtrees(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let mut ans = vec![];
        let mut cnt = HashMap::new();

        Self::dfs(&mut ans, &mut cnt, root);

        ans
    }

    fn dfs(
        ans: &mut Vec<Option<Rc<RefCell<TreeNode>>>>,
        cnt: &mut HashMap<String, i32>,
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> String {
        if let Some(r) = root {
            let left = Self::dfs(ans, cnt, r.borrow().left.clone());
            let right = Self::dfs(ans, cnt, r.borrow().right.clone());
            let key = format!("{} {} {}", r.borrow().val, left, right);

            *cnt.entry(key.clone()).or_insert(0) += 1;

            if let Some(&v) = cnt.get(&key) {
                if v == 2 {
                    ans.push(Some(r));
                }
            }
            key
        } else {
            ",".to_string()
        }
    }
}

fn main() {
    {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
            }))),
        })));

        let ans = vec![
            Some(Rc::new(RefCell::new(TreeNode::new(4)))),
            Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                right: None,
            }))),
        ];
        assert_eq!(Solution::find_duplicate_subtrees(root), ans);
    }

    {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
        })));

        let ans = vec![Some(Rc::new(RefCell::new(TreeNode::new(1))))];
        assert_eq!(Solution::find_duplicate_subtrees(root), ans);
    }

    {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                right: None,
            }))),
        })));

        let ans = vec![
            Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                right: None,
            }))),
        ];
        assert_eq!(Solution::find_duplicate_subtrees(root), ans);
    }
}
