/*
 * @Date: 2022-09-02
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-09-02
 * @FilePath: /algorithm/687_longest_univalue_path/longest_univalue_path.rs
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

pub fn longest_univalue_path(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, ans: &mut i32) -> (Option<i32>, i32) {
        if let Some(t) = root {
            let val_root = t.borrow().val;
            let (lval, llen) = dfs(t.borrow_mut().left.take(), ans);
            let (rval, rlen) = dfs(t.borrow_mut().right.take(), ans);
            let llen = if let Some(t) = lval {
                if t == val_root {
                    llen
                } else {
                    0
                }
            } else {
                0
            };
            let rlen = if let Some(t) = rval {
                if t == val_root {
                    rlen
                } else {
                    0
                }
            } else {
                0
            };
            *ans = (*ans).max(1 + llen + rlen);
            (Some(val_root), 1 + llen.max(rlen))
        } else {
            (None, 0)
        }
    }
    let mut ans = 1;
    dfs(root, &mut ans).1.max(ans) - 1
}

fn main() {
    {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 5,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
            }))),
        })));
        let ans = 2;
        assert_eq!(longest_univalue_path(root), ans);
    }

    {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 5,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
            }))),
        })));
        let ans = 2;
        assert_eq!(longest_univalue_path(root), ans);
    }
}
