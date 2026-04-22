/*
 * @Date: 2024-03-12
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-03-12
 * @FilePath: /algorithm/rust/1261_FindElements/FindElements.rs
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

struct FindElements {
    root: Option<Rc<RefCell<TreeNode>>>,
}

impl FindElements {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        Self { root }
    }

    fn find(&self, target: i32) -> bool {
        let target = target + 1;
        let mut cur = self.root.clone(); // 从根节点出发
        for i in (0..target.ilog2()).rev() {
            // 从次高位开始枚举
            let bit = (target >> i) & 1; // target 第 i 位的比特值
            cur = if bit == 0 {
                cur.unwrap().borrow().left.clone()
            } else {
                cur.unwrap().borrow().right.clone()
            };
            if cur.is_none() {
                // 走到空节点，说明 target 不在二叉树中
                return false;
            }
        }
        true // 没有走到空节点，说明 target 在二叉树中
    }
}

fn main() {
    let obj = FindElements::new(Some(Rc::new(RefCell::new(TreeNode {
        val: -1,
        left: None,
        right: Some(Rc::new(RefCell::new(TreeNode::new(-1)))),
    }))));

    assert_eq!(obj.find(1), false);
    assert_eq!(obj.find(2), true);
}
