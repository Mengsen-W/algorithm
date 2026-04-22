/*
 * @Date: 2024-02-24
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-02-24
 * @FilePath: /algorithm/rust/2476_closest_nodes/closest_nodes.rs
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
    pub fn closest_nodes(root: Option<Rc<RefCell<TreeNode>>>, queries: Vec<i32>) -> Vec<Vec<i32>> {
        fn dfs(node: Option<&Rc<RefCell<TreeNode>>>, a: &mut Vec<i32>) {
            if let Some(x) = node {
                let x = x.borrow();
                dfs(x.left.as_ref(), a);
                a.push(x.val);
                dfs(x.right.as_ref(), a);
            }
        }
        let mut a = Vec::new();
        dfs(root.as_ref(), &mut a);

        let n = a.len();
        let mut ans = Vec::new();
        for q in queries {
            let j = a.partition_point(|&x| x < q);
            let mx = if j < n { a[j] } else { -1 };
            let mn = if j < n && a[j] == q {
                q
            } else if j > 0 {
                a[j - 1]
            } else {
                -1
            };
            ans.push(vec![mn, mx]);
        }
        ans
    }
}

fn main() {
    let tests = vec![
        (
            Some(Rc::new(RefCell::new(TreeNode {
                val: 6,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 13,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 15,
                        left: Some(Rc::new(RefCell::new(TreeNode::new(14)))),
                        right: None,
                    }))),
                }))),
            }))),
            vec![2, 5, 16],
            vec![vec![2, 2], vec![4, 6], vec![15, -1]],
        ),
        (
            Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
            }))),
            vec![3],
            vec![vec![-1, 4]],
        ),
    ];

    for (root, queries, ans) in tests {
        assert_eq!(Solution::closest_nodes(root, queries), ans);
    }
}
