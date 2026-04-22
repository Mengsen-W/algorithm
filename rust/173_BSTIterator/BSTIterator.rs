/*
 * @Date: 2021-03-28 10:19:53
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-03-28 10:43:25
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
struct BSTIterator {
    nodes: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl BSTIterator {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut nodes = Vec::new();
        let mut stk = Vec::new();
        let mut root = root;
        while root.is_some() || !stk.is_empty() {
            while root.is_some() {
                let node = root.take().unwrap();
                nodes.push(node.borrow().val);
                root = node.borrow_mut().left.take();
                stk.push(node);
            }
            root = stk.pop();
            root = root.unwrap().borrow_mut().right.take();
        }

        nodes.sort_by(|a, b| b.cmp(a));

        BSTIterator { nodes }
    }

    fn next(&mut self) -> i32 {
        self.nodes.pop().unwrap()
    }

    fn has_next(&self) -> bool {
        !self.nodes.is_empty()
    }
}
/**
 * Your BSTIterator object will be instantiated and called as such:
 * let obj = BSTIterator::new(root);
 * let ret_1: i32 = obj.next();
 * let ret_2: bool = obj.has_next();
 */

fn main() {
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 7,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: None,
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 15,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 9,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 20,
                left: None,
                right: None,
            }))),
        }))),
    })));
    let mut it = BSTIterator::new(root);
    assert_eq!(it.next(), 3);
    assert_eq!(it.next(), 7);
    assert!(it.has_next());
    assert_eq!(it.next(), 9);
    assert!(it.has_next());
    assert_eq!(it.next(), 15);
    assert!(it.has_next());
    assert_eq!(it.next(), 20);
    assert!(!it.has_next());
}
