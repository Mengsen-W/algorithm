/*
 * @Date: 2021-07-28 17:36:09
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-07-28 19:55:09
 */

// use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(
        val: i32,
        left: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>>,
    ) -> Self {
        TreeNode { val, left, right }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Solution {}

pub fn distance_k(
    root: Option<Rc<RefCell<TreeNode>>>,
    target: Option<Rc<RefCell<TreeNode>>>,
    k: i32,
) -> Vec<i32> {
    if root.is_none() || target.is_none() {
        return vec![];
    }
    let (root, target) = (root.unwrap(), target.unwrap());
    let mut parent_map = HashMap::new();
    dfs_parent(root, &mut parent_map);
    let mut ans = vec![];
    dfs_ans(&target.borrow(), -1, 0, k, &parent_map, &mut ans);
    ans
}

fn dfs_parent(cur: Rc<RefCell<TreeNode>>, parent_map: &mut HashMap<i32, Rc<RefCell<TreeNode>>>) {
    if let Some(nd) = cur.borrow().left.as_ref() {
        parent_map.insert(nd.borrow().val, cur.clone());
        dfs_parent(nd.clone(), parent_map);
    }
    if let Some(nd) = cur.borrow().right.as_ref() {
        parent_map.insert(nd.borrow().val, cur.clone());
        dfs_parent(nd.clone(), parent_map);
    }
}

fn dfs_ans(
    cur: &TreeNode,
    from: i32,
    depth: i32,
    k: i32,
    parent_map: &HashMap<i32, Rc<RefCell<TreeNode>>>,
    ans: &mut Vec<i32>,
) {
    if depth == k {
        ans.push(cur.val);
        return;
    }
    if let Some(nd) = cur.left.as_ref() {
        if nd.borrow().val != from {
            dfs_ans(&nd.borrow(), cur.val, depth + 1, k, parent_map, ans);
        }
    }
    if let Some(nd) = cur.right.as_ref() {
        if nd.borrow().val != from {
            dfs_ans(&nd.borrow(), cur.val, depth + 1, k, parent_map, ans);
        }
    }
    if let Some(nd) = parent_map.get(&cur.val) {
        if nd.borrow().val != from {
            dfs_ans(&nd.borrow(), cur.val, depth + 1, k, parent_map, ans);
        }
    }
}

fn main() {
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 6,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 7,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                }))),
            }))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 0,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 8,
                left: None,
                right: None,
            }))),
        }))),
    })));

    let target = Some(Rc::new(RefCell::new(TreeNode {
        val: 5,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 6,
            left: None,
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 7,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: None,
                right: None,
            }))),
        }))),
    })));

    assert_eq!(distance_k(root, target, 2), vec![7, 4, 1],);
}
