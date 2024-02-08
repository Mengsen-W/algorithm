/*
 * @Date: 2021-05-17 08:36:19
 * @Author: Mengsen Wang
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-02-08
 */

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

fn is_cousins_bfs(root: Option<Rc<RefCell<TreeNode>>>, x: i32, y: i32) -> bool {
    // Passed 0ms 2mb
    let mut que = std::collections::VecDeque::new();
    if root.is_some() {
        que.push_back(root);
    }
    while !que.is_empty() {
        let mut found = Vec::new();
        for _ in 0..que.len() {
            if let Some(node) = que.pop_front() {
                if let Some(node) = node {
                    let left = node.borrow_mut().left.take();
                    let right = node.borrow_mut().right.take();
                    let parent_val = node.borrow().val;
                    if left.is_some() {
                        let val = left.as_ref().unwrap().borrow().val;
                        if val == x || val == y {
                            found.push(parent_val);
                        }
                        que.push_back(left);
                    }
                    if right.is_some() {
                        let val = right.as_ref().unwrap().borrow().val;
                        if val == x || val == y {
                            found.push(parent_val);
                        }
                        que.push_back(right);
                    }
                }
            }
        }
        if !found.is_empty() {
            return if found.len() == 1 {
                false
            } else {
                found[0] != found[1]
            };
        }
    }
    false
}

fn is_cousins_dfs(root: Option<Rc<RefCell<TreeNode>>>, x: i32, y: i32) -> bool {
    type Tree = Option<Rc<RefCell<TreeNode>>>;
    fn find(root: Tree, x: i32, p: Tree, l: i32) -> Option<(Tree, i32)> {
        let l1 = l + 1;
        if let Some(node) = &root {
            let node = node.borrow();
            if node.val == x {
                return Some((p, l1));
            } else {
                if let Some(r) = find(node.left.clone(), x, root.clone(), l1) {
                    return Some(r);
                }
                if let Some(r) = find(node.right.clone(), x, root.clone(), l1) {
                    return Some(r);
                }
            }
        }
        None
    }

    let (f1, l1) = find(root.clone(), x, None, 0).unwrap();
    let (f2, l2) = find(root.clone(), y, None, 0).unwrap();
    f1 != f2 && l1 == l2
}

fn main() {
    let tests = vec![
        (
            Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 4,
                        left: None,
                        right: None,
                    }))),
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None,
                }))),
            }))),
            4,
            3,
            false,
        ),
        (
            Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 4,
                        left: None,
                        right: None,
                    }))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 5,
                        left: None,
                        right: None,
                    }))),
                }))),
            }))),
            5,
            4,
            true,
        ),
        (
            Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 4,
                        left: None,
                        right: None,
                    }))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None,
                }))),
            }))),
            2,
            3,
            false,
        ),
    ];

    for (root, x, y, ans) in tests {
        assert_eq!(is_cousins_bfs(root.clone(), x, y), ans);
        // assert_eq!(is_cousins_dfs(root.clone(), x, y), ans);
    }

    {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                }))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            }))),
        })));
        assert!(!is_cousins_bfs(root.clone(), 4, 3));
        // assert!(!is_cousins_dfs(root.clone(), 4, 3));
    }
    {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: None,
                    right: None,
                }))),
            }))),
        })));
        assert!(is_cousins_bfs(root.clone(), 5, 4));
        // assert!(is_cousins_dfs(root.clone(), 5, 4));
    }
    {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            }))),
        })));
        assert!(!is_cousins_bfs(root.clone(), 2, 3));
        // assert!(!is_cousins_dfs(root.clone(), 4, 3));
    }
}
