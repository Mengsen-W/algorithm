/*
 * @Date: 2022-08-22
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-08-22
 * @FilePath: /algorithm/655_print_tree/print_tree.rs
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
    pub fn print_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<String>> {
        use std::collections::VecDeque;
        fn get_height(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
            if root.is_none() {
                0
            } else {
                1 + (get_height(&root.as_ref().unwrap().borrow().left.clone())
                    .max(get_height(&root.as_ref().unwrap().borrow().right.clone())))
            }
        }

        let height = get_height(&root) - 1;
        let (m, n) = (height + 1, (2 << height) - 1);
        let (mut row, cols) = (0, vec!["".to_string(); n as usize]);
        let mut ret = vec![cols; m as usize];
        let (mut queue, mut indices) = (VecDeque::new(), VecDeque::new());

        queue.push_back(root);
        indices.push_back(vec![0, n - 1]);
        while !queue.is_empty() {
            let size = queue.len();
            for _ in 0..size {
                let (curr, index) = (queue.pop_front().unwrap(), indices.pop_front().unwrap());
                if curr.is_none() {
                    continue;
                }
                let (l, r) = (index[0], index[1]);
                let m = l + ((r - l) >> 1);
                ret[row][m] = curr.as_ref().unwrap().borrow().val.to_string();
                queue.push_back(curr.as_ref().unwrap().borrow().left.clone());
                queue.push_back(curr.as_ref().unwrap().borrow().right.clone());
                indices.push_back(vec![l, (m as i32 - 1) as usize]);
                indices.push_back(vec![m + 1, r]);
            }
            row += 1;
        }
        ret
    }
}

fn main() {
    {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
            right: None,
        })));
        let ans = [["", "1", ""], ["2", "", ""]]
            .iter()
            .map(|x| {
                x.to_vec()
                    .iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>()
            })
            .collect::<Vec<Vec<String>>>();
        assert_eq!(Solution::print_tree(root), ans);
    }

    {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        })));
        let ans = [
            ["", "", "", "1", "", "", ""],
            ["", "2", "", "", "", "3", ""],
            ["", "", "4", "", "", "", ""],
        ]
        .iter()
        .map(|x| {
            x.to_vec()
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>();
        assert_eq!(Solution::print_tree(root), ans);
    }
}
