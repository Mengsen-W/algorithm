/*
 * @Date: 2023-05-22
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-05-22
 * @FilePath: /algorithm/rust/1080_sufficient_subset/sufficient_subset.rs
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

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn sufficient_subset(
        root: Option<Rc<RefCell<TreeNode>>>,
        limit: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        fn dfs(
            root: Option<Rc<RefCell<TreeNode>>>,
            limit: i32,
            sum: i32,
        ) -> (Option<Rc<RefCell<TreeNode>>>, i32) {
            if let Some(root) = root {
                let mut child_sum = root.borrow().val;
                let mut left = dfs(root.borrow_mut().left.take(), limit, sum + child_sum);
                let mut right = dfs(root.borrow_mut().right.take(), limit, sum + child_sum);
                child_sum += if left.1 == i32::MIN && right.1 == i32::MIN {
                    0
                } else {
                    left.1.max(right.1)
                };
                if sum + child_sum < limit {
                    (None, child_sum)
                } else {
                    root.borrow_mut().left = left.0.take();
                    root.borrow_mut().right = right.0.take();
                    (Some(root), child_sum)
                }
            } else {
                (None, i32::MIN)
            }
        }
        dfs(root, limit, 0).0
    }
}

fn main() {
    {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(8)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: -99,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(-99)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(-99)))),
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: -99,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(12)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(13)))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 7,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(-99)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(14)))),
                }))),
            }))),
        })));
        let limit = 1;
        let ans = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(8)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
                }))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 7,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode::new(14)))),
                }))),
            }))),
        })));
        assert_eq!(Solution::sufficient_subset(root, limit), ans);
    }

    {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 11,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                }))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 8,
                left: Some(Rc::new(RefCell::new(TreeNode::new(17)))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                }))),
            }))),
        })));
        let limit = 22;
        let ans = Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 11,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
                    right: None,
                }))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 8,
                left: Some(Rc::new(RefCell::new(TreeNode::new(17)))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
                    right: None,
                }))),
            }))),
        })));
        assert_eq!(Solution::sufficient_subset(root, limit), ans);
    }

    {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(-5)))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: -3,
                left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                right: None,
            }))),
        })));
        let limit = -1;
        let ans = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: -3,
                left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                right: None,
            }))),
        })));
        assert_eq!(Solution::sufficient_subset(root, limit), ans);
    }
}
