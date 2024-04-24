/*
 * @Date: 2024-04-24
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-04-24
 * @FilePath: /algorithm/rust/2385_amount_of_time/amount_of_time.rs
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
use std::collections::{HashMap, HashSet, VecDeque};
use std::rc::Rc;

impl Solution {
    pub fn amount_of_time(root: Option<Rc<RefCell<TreeNode>>>, start: i32) -> i32 {
        let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut visited: HashSet<i32> = HashSet::new();

        fn dfs(node_opt: Option<&Rc<RefCell<TreeNode>>>, graph: &mut HashMap<i32, Vec<i32>>) {
            if let Some(node_ref) = node_opt {
                let node = node_ref.borrow();
                if let Some(ref left) = &node.left {
                    graph
                        .entry(node.val)
                        .or_insert_with(Vec::new)
                        .push(left.borrow().val);
                    graph
                        .entry(left.borrow().val)
                        .or_insert_with(Vec::new)
                        .push(node.val);
                    dfs(Some(&left), graph);
                }
                if let Some(ref right) = &node.right {
                    graph
                        .entry(node.val)
                        .or_insert_with(Vec::new)
                        .push(right.borrow().val);
                    graph
                        .entry(right.borrow().val)
                        .or_insert_with(Vec::new)
                        .push(node.val);
                    dfs(Some(&right), graph);
                }
            }
        }

        if let Some(node) = root.as_ref() {
            dfs(Some(&node), &mut graph);
        }

        let mut q = VecDeque::new();
        let mut time = 0;
        q.push_back((start, 0));
        visited.insert(start);
        while let Some((node_val, curr_time)) = q.pop_front() {
            time = curr_time;
            if let Some(children) = graph.get(&node_val) {
                for &child_val in children {
                    if !visited.contains(&child_val) {
                        q.push_back((child_val, time + 1));
                        visited.insert(child_val);
                    }
                }
            }
        }
        time
    }
}

fn main() {
    let tests = vec![
        (
            Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 4,
                        left: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
                        right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
                    }))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(10)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
                }))),
            }))),
            3,
            4,
        ),
        (Some(Rc::new(RefCell::new(TreeNode::new(1)))), 1, 0),
    ];

    for (root, start_val, ans) in tests {
        assert_eq!(Solution::amount_of_time(root, start_val), ans);
    }
}
