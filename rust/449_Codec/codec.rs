/*
 * @Date: 2023-09-04
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-09-04
 * @FilePath: /algorithm/rust/449_Codec/codec.rs
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
use std::str::SplitWhitespace;
struct Codec {}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    fn new() -> Self {
        Self {}
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        self.pre_order(&root)
    }

    fn pre_order(&self, root: &Option<Rc<RefCell<TreeNode>>>) -> String {
        if let Some(node) = root {
            format!(
                "{} {} {}",
                node.borrow().val.to_string(),
                self.pre_order(&node.borrow().left),
                self.pre_order(&node.borrow().right),
            )
        } else {
            ",".to_string()
        }
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        if data.len() == 0 {
            return None;
        }
        let mut vec = data.split_whitespace();
        self.constructor(&mut vec)
    }

    fn constructor(&self, iter: &mut SplitWhitespace) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(val) = iter.next() {
            if val != "," {
                let node = Rc::new(RefCell::new(TreeNode::new(val.parse::<i32>().unwrap())));
                node.borrow_mut().left = self.constructor(iter);
                node.borrow_mut().right = self.constructor(iter);
                Some(node)
            } else {
                None
            }
        } else {
            None
        }
    }
}

fn main() {
    let tests = vec!["2 1 3 , , , ,", ","];
    for test in tests {
        let test = test.to_string();
        assert_eq!(
            Codec::new().serialize(Codec::new().deserialize(test.clone())),
            test
        );
    }
}
