/*
 * @Date: 2022-03-18 23:49:21
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-03-19 00:39:47
 * @FilePath: /algorithm/606_tree2str/tree2str.rs
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

struct Solution;

impl Solution {
    pub fn tree2str(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        match root {
            None => "".to_string(),
            Some(n) => {
                let n = n.borrow();
                if n.left == None && n.right == None {
                    return format!("{}", n.val);
                }
                let left = match n.left.clone() {
                    None => "()".to_string(),
                    l @ Some(_) => format!("({})", Solution::tree2str(l)),
                };
                let right = match n.right.clone() {
                    None => "".to_string(),
                    r @ Some(_) => format!("({})", Solution::tree2str(r)),
                };
                format!("{}{}{}", n.val, left, right)
            }
        }
    }
}

// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//     pub val: i32,
//     pub left: Option<Rc<RefCell<TreeNode>>>,
//     pub right: Option<Rc<RefCell<TreeNode>>>,
// }

// impl TreeNode {
//     #[inline]
//     pub fn new(val: i32) -> Self {
//         TreeNode {
//             val,
//             left: None,
//             right: None,
//         }
//     }
// }

// impl fmt::Display for TreeNode {
//     fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), Error> {
//         fmt.write_fmt(format_args!("{}", self.val));
//         if self.left.is_none() && self.right.is_none() { return Ok(()); }
//         fmt.write_char('(');
//         if let Some(n) = self.left.as_ref() {
//             fmt.write_str(&n.borrow().to_string());
//         }
//         fmt.write_char(')');
//         if let Some(n) = self.right.as_ref() {
//             fmt.write_char('(');
//             fmt.write_str(&n.borrow().to_string());
//             fmt.write_char(')');
//         }
//         Ok(())
//     }
// }

// use std::rc::Rc;
// use std::cell::RefCell;
// use std::fmt;
// use std::fmt::{Formatter, Error, Write as MyWright};

// pub fn tree2str(t: Option<Rc<RefCell<TreeNode>>>) -> String {
//     if let Some(x) = t {
//         x.borrow().to_string()
//     } else { "".to_string() }
// }

fn main() {
    assert_eq!(
        Solution::tree2str(Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None
                }))),
                right: None
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None
            }))),
        })))),
        "1(2(4))(3)".to_string()
    );
    assert_eq!(
        Solution::tree2str(Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None
            }))),
        })))),
        "1(2()(4))(3)".to_string()
    );
}
