/*
 * @Date: 2024-02-22
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-02-22
 * @FilePath: /algorithm/rust/889_construct_from_pre_post/construct_from_pre_post.rs
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
use std::rc::Rc;

impl Solution {
    pub fn construct_from_pre_post(
        preorder: Vec<i32>,
        postorder: Vec<i32>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let n = preorder.len();
        let mut index = vec![0; n + 1];
        for (i, &x) in postorder.iter().enumerate() {
            index[x as usize] = i;
        }

        fn dfs(
            preorder: &Vec<i32>,
            pre_l: usize,
            pre_r: usize,
            postorder: &Vec<i32>,
            post_l: usize,
            post_r: usize,
            index: &Vec<usize>,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            if pre_l == pre_r {
                return None;
            }
            if pre_l + 1 == pre_r {
                return Some(Rc::new(RefCell::new(TreeNode::new(preorder[pre_l]))));
            }
            let left_size = index[preorder[pre_l + 1] as usize] - post_l + 1;
            let left = dfs(
                preorder,
                pre_l + 1,
                pre_l + 1 + left_size,
                postorder,
                post_l,
                post_l + left_size,
                index,
            );
            let right = dfs(
                preorder,
                pre_l + 1 + left_size,
                pre_r,
                postorder,
                post_l + left_size,
                post_r - 1,
                index,
            );
            Some(Rc::new(RefCell::new(TreeNode {
                val: preorder[pre_l],
                left,
                right,
            })))
        }
        dfs(&preorder, 0, n, &postorder, 0, n, &index)
    }
}

fn main() {
    let tests = vec![
        (
            vec![1, 2, 4, 5, 3, 6, 7],
            vec![4, 5, 2, 6, 7, 3, 1],
            Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
                }))),
            }))),
        ),
        (
            vec![1],
            vec![1],
            Some(Rc::new(RefCell::new(TreeNode::new(1)))),
        ),
    ];

    for (preorder, postorder, ans) in tests {
        assert_eq!(Solution::construct_from_pre_post(preorder, postorder), ans);
    }
}
