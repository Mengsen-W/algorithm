/*
 * @Date: 2021-11-26 01:23:59
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-11-26 02:15:41
 */

use std::cell::RefCell;
use std::rc::Rc;

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
    pub fn search_bst(
        mut root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return None;
        }

        let v = root.as_ref().unwrap().borrow().val;
        if v.eq(&val) {
            return root;
        } else if v.gt(&val) {
            return Self::search_bst(root.as_mut().unwrap().borrow_mut().left.take(), val);
        } else if v.lt(&val) {
            return Self::search_bst(root.as_mut().unwrap().borrow_mut().right.take(), val);
        }

        unreachable!()
    }
}

fn main() {
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 4,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
    })));
    let result = Solution::search_bst(root, 2);
    assert_eq!(result.as_ref().unwrap().borrow().val, 2);
    assert_eq!(
        result
            .as_ref()
            .unwrap()
            .borrow()
            .left
            .as_ref()
            .unwrap()
            .borrow()
            .val,
        1
    );
    assert_eq!(
        result
            .as_ref()
            .unwrap()
            .borrow()
            .right
            .as_ref()
            .unwrap()
            .borrow()
            .val,
        3
    );
}
