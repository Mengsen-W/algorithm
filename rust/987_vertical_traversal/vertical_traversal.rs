/*
 * @Date: 2021-07-31 00:49:38
 * @Author: Mengsen Wang
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-02-13
 */

use std::cell::RefCell;
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

pub struct Solution {}

impl Solution {
    pub fn vertical_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if root.is_none() {
            return vec![];
        }

        fn dfs(arr: &mut Vec<(i32, i32, i32)>, cur: &TreeNode, row: i32, col: i32) {
            arr.push((col, row, cur.val));
            if let Some(nd) = cur.left.as_ref() {
                dfs(arr, &nd.borrow(), row + 1, col - 1);
            }
            if let Some(nd) = cur.right.as_ref() {
                dfs(arr, &nd.borrow(), row + 1, col + 1);
            }
        }

        let root = root.unwrap();
        let mut arr = vec![];
        dfs(&mut arr, &root.borrow(), 0, 0);
        arr.sort();
        arr.into_iter()
            .fold((i32::MIN, vec![]), |(prev_c, mut ans), (c, _, v)| {
                if prev_c != c {
                    ans.push(vec![v]);
                } else {
                    ans.last_mut().unwrap().push(v);
                }
                (c, ans)
            })
            .1
    }
}

fn main() {
    let tests = vec![
        (
            Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 9,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 20,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 15,
                        left: None,
                        right: None,
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 7,
                        left: None,
                        right: None,
                    }))),
                }))),
            }))),
            vec![vec![9], vec![3, 15], vec![20], vec![7]],
        ),
        (
            Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 4,
                        left: None,
                        right: None,
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 5,
                        left: None,
                        right: None,
                    }))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 6,
                        left: None,
                        right: None,
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 7,
                        left: None,
                        right: None,
                    }))),
                }))),
            }))),
            vec![vec![4], vec![2], vec![1, 5, 6], vec![3], vec![7]],
        ),
        (
            Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 4,
                        left: None,
                        right: None,
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 6,
                        left: None,
                        right: None,
                    }))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 5,
                        left: None,
                        right: None,
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 7,
                        left: None,
                        right: None,
                    }))),
                }))),
            }))),
            vec![vec![4], vec![2], vec![1, 5, 6], vec![3], vec![7]],
        ),
    ];

    for (root, ans) in tests {
        assert_eq!(Solution::vertical_traversal(root), ans);
    }
}
