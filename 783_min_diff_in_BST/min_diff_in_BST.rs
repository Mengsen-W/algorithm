/*
 * @Date: 2021-04-13 08:42:01
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-04-13 09:29:13
 */

use std::cell::RefCell;
use std::rc::Rc;

#[allow(dead_code)]
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

fn min_diff_in_bst(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut ans = i32::MAX;
    let mut pre = -1;

    inorder(root, &mut ans, &mut pre);

    ans
}

fn inorder(root: Option<Rc<RefCell<TreeNode>>>, ans: &mut i32, pre: &mut i32) {
    if let Some(r) = root {
        inorder(r.borrow().left.clone(), ans, pre);

        if *pre >= 0 {
            *ans = (*ans).min(r.borrow().val - *pre);
        }

        *pre = r.borrow().val;

        inorder(r.borrow().right.clone(), ans, pre);
    }
}

fn min_diff_in_bst1(mut root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if root.is_none() {
        return 0;
    }

    let mut stack = Vec::new();
    let mut pre = -1;
    let mut ans = i32::MAX;

    while stack.len() > 0 || root.is_some() {
        while let Some(r) = root {
            root = r.borrow().left.clone();
            stack.push(r);
        }

        let r = stack.pop().unwrap();

        if pre >= 0 {
            ans = ans.min(r.borrow().val - pre);
        }
        pre = r.borrow().val;

        root = r.borrow().right.clone();
    }

    ans
}

fn main() {
    let root = TreeNode {
        val: 4,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            }))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 6,
            left: None,
            right: None,
        }))),
    };
    assert_eq!(min_diff_in_bst1(Some(Rc::new(RefCell::new(root)))), 1);

    let root = TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 0,
            left: None,
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 48,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 12,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 49,
                left: None,
                right: None,
            }))),
        }))),
    };
    assert_eq!(min_diff_in_bst(Some(Rc::new(RefCell::new(root)))), 1);
}
