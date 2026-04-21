/*
 * @Date: 2023-02-03
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-02-03
 * @FilePath: /algorithm/rust/1145_btree_game_winning_move/btree_game_winning_move.rs
 */

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

use std::cell::RefCell;
use std::rc::Rc;
pub fn btree_game_winning_move(root: Option<Rc<RefCell<TreeNode>>>, n: i32, x: i32) -> bool {
    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, x: i32, tl: &mut i32, tr: &mut i32) -> i32 {
        if let Some(n) = node {
            let left = dfs(&n.borrow().left, x, tl, tr);
            let right = dfs(&n.borrow().right, x, tl, tr);
            if x == n.borrow().val {
                *tl = left;
                *tr = right;
            }
            left + right + 1
        } else {
            0
        }
    }
    let mut l = 0;
    let mut r = 0;
    dfs(&root, x, &mut l, &mut r);
    if l > n / 2 || r > n / 2 || (n - l - r - 1) > n / 2 {
        return true;
    }
    false
}

fn main() {
    {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 8,
                        left: None,
                        right: None,
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 9,
                        left: None,
                        right: None,
                    }))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 10,
                        left: None,
                        right: None,
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 11,
                        left: None,
                        right: None,
                    }))),
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 6,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 7,
                    left: None,
                    right: None,
                }))),
            }))),
        })));
        let n = 11;
        let x = 3;
        let ans = true;
        assert_eq!(btree_game_winning_move(root, n, x), ans);
    }

    {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            }))),
        })));
        let n = 3;
        let x = 1;
        let ans = false;
        assert_eq!(btree_game_winning_move(root, n, x), ans);
    }
}
