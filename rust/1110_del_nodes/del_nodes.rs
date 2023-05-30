/*
 * @Date: 2023-05-30
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-05-30
 * @FilePath: /algorithm/rust/1110_del_nodes/del_nodes.rs
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
use std::collections::HashSet;
use std::rc::Rc;

pub fn del_nodes(
    root: Option<Rc<RefCell<TreeNode>>>,
    to_delete: Vec<i32>,
) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
    let to_delete = to_delete.into_iter().collect::<HashSet<_>>();

    fn dfs(
        root: Option<Rc<RefCell<TreeNode>>>,
        to_delete: &HashSet<i32>,
        res: &mut Vec<Option<Rc<RefCell<TreeNode>>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root {
            let mut tr = node.borrow_mut();
            tr.left = dfs(tr.left.take(), to_delete, res);
            tr.right = dfs(tr.right.take(), to_delete, res);

            if to_delete.contains(&tr.val) {
                if let Some(left) = tr.left.take() {
                    res.push(Some(left));
                }
                if let Some(right) = tr.right.take() {
                    res.push(Some(right));
                }
                None
            } else {
                Some(Rc::clone(&node))
            }
        } else {
            None
        }
    }

    let mut res = Vec::new();
    dfs(root.clone(), &to_delete, &mut res);

    let tr = root.clone();
    if !to_delete.contains(&tr.unwrap().borrow_mut().val) {
        res.push(root);
    }
    res
}

fn main() {
    {
        let root = Some(Rc::new(RefCell::new(TreeNode {
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
        })));
        let to_delete = vec![3, 5];
        let res = vec![
            Some(Rc::new(RefCell::new(TreeNode::new(6)))),
            Some(Rc::new(RefCell::new(TreeNode::new(7)))),
            Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                    right: None,
                }))),
                right: None,
            }))),
        ];
        assert_eq!(del_nodes(root, to_delete), res);
    }

    {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
        })));
        let to_delete = vec![3];
        let res = vec![Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
        })))];
        assert_eq!(del_nodes(root, to_delete), res);
    }
}
