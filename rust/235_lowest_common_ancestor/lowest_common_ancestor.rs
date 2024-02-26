/*
 * @Date: 2024-02-25
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-02-25
 * @FilePath: /algorithm/rust/235_lowest_common_ancestor/lowest_common_ancestor.rs
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

impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let x = root.as_ref().unwrap();
        let x_val = x.borrow().val;
        let p_val = p.as_ref().unwrap().borrow().val;
        let q_val = q.as_ref().unwrap().borrow().val;
        if p_val < x_val && q_val < x_val {
            // p 和 q 都在左子树
            return Self::lowest_common_ancestor(x.borrow_mut().left.take(), p, q);
        }
        if p_val > x_val && q_val > x_val {
            // p 和 q 都在右子树
            return Self::lowest_common_ancestor(x.borrow_mut().right.take(), p, q);
        }
        root // 其它
    }
}

fn main() {}
