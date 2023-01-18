/*
 * @Date: 2022-07-25
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-07-25
 * @FilePath: /algorithm/919_CBT_inserter/CBT_inserter.rs
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
struct CBTInserter {
    p: Vec<Option<Rc<RefCell<TreeNode>>>>,
}

impl CBTInserter {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut p = vec![root];
        let mut i = 0;
        while i < p.len() {
            let left = p[i].as_ref().unwrap().borrow().left.clone();
            if left.is_some() {
                p.push(left);
            }
            let right = p[i].as_ref().unwrap().borrow().right.clone();
            if right.is_some() {
                p.push(right);
            }
            i += 1;
        }
        Self { p }
    }

    fn insert(&mut self, v: i32) -> i32 {
        let n = self.p.len();
        self.p.push(Some(Rc::new(RefCell::new(TreeNode::new(v)))));
        let mut father = self.p[(n - 1) >> 1].as_ref().unwrap().borrow_mut();
        if n & 1 == 1 {
            father.left = self.p[n].clone();
        } else {
            father.right = self.p[n].clone();
        }
        father.val
    }

    fn get_root(&self) -> Option<Rc<RefCell<TreeNode>>> {
        self.p[0].clone()
    }
}

fn main() {
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        right: None,
    })));
    let mut c = CBTInserter::new(root.clone());
    assert_eq!(c.insert(3), 1);
    assert_eq!(c.insert(4), 2);
    assert_eq!(c.get_root(), root);
}
