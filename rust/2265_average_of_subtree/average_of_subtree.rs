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
    pub fn average_of_subtree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut ans = 0;
        Self::dfs(&root, &mut ans);
        ans
    }
    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, ans: &mut i32) -> (i32, i32) {
        match node {
            None => (0, 0),
            Some(node) => {
                let node = node.borrow();
                let (leftSum, leftSize) = Self::dfs(&node.left, ans);
                let (rightSum, rightSize) = Self::dfs(&node.right, ans);
                let Size = leftSize + rightSize + 1;
                let Sum = leftSum + rightSum + node.val;
                if Size > 0 && Sum / Size == node.val {
                    *ans += 1;
                }
                (Sum, Size)
            }
        }
    }
}

fn main() {
    let tests = vec![
        (
            Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 8,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
                }))),
            }))),
            5,
        ),
        (Some(Rc::new(RefCell::new(TreeNode::new(1)))), 1),
    ];

    for (root, ans) in tests {
        assert_eq!(Solution::average_of_subtree(root), ans);
    }
}
