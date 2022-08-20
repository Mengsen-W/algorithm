/*
 * @Date: 2022-08-20
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-08-20
 * @FilePath: /algorithm/654_construct_maximum_binary_tree/construct_maximum_binary_tree.rs
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
    pub fn construct_maximum_binary_tree(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn build_tree(lo: i32, hi: i32, nums: &Vec<i32>) -> Rc<RefCell<TreeNode>> {
            if lo == hi {
                return Rc::new(RefCell::new(TreeNode::new(nums[lo as usize])));
            }
            let max_idx = (lo + 1..=hi).fold(lo, |max_idx, i| {
                if nums[i as usize] > nums[max_idx as usize] {
                    i
                } else {
                    max_idx
                }
            });
            let node = Rc::new(RefCell::new(TreeNode::new(nums[max_idx as usize])));
            if max_idx - 1 >= lo {
                node.borrow_mut().left = Some(build_tree(lo, max_idx - 1, nums));
            }
            if max_idx + 1 <= hi {
                node.borrow_mut().right = Some(build_tree(max_idx + 1, hi, nums));
            }
            node
        }
        Some(build_tree(0 as i32, nums.len() as i32 - 1, &nums))
    }
}

fn main() {
    {
        let nums = vec![3, 2, 1, 6, 0, 5];
        let ans = Some(Rc::new(RefCell::new(TreeNode {
            val: 6,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 5,
                left: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
                right: None,
            }))),
        })));
        assert_eq!(Solution::construct_maximum_binary_tree(nums), ans);
    }

    {
        let nums = vec![3, 2, 1];
        let ans = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            }))),
        })));
        assert_eq!(Solution::construct_maximum_binary_tree(nums), ans);
    }
}
