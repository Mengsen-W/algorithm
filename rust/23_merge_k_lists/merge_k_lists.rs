/*
 * @Date: 2023-08-12
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-08-12
 * @FilePath: /algorithm/rust/23_merge_k_lists/merge_k_lists.rs
 */

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution;

use std::{
    cmp::{Ord, PartialOrd, Reverse},
    collections::BinaryHeap,
};

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.val.cmp(&other.val)
    }
}

impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.val.cmp(&other.val))
    }
}

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut node = None;
        let mut cur = &mut node;
        let mut heap = BinaryHeap::new();
        for list in lists.into_iter() {
            if let Some(x) = list {
                heap.push(Reverse(x));
            }
        }
        while let Some(mut x) = heap.pop() {
            if let Some(y) = x.0.next.take() {
                heap.push(Reverse(y));
            }
            cur = &mut cur.insert(x.0).next;
        }
        node
    }
}

fn main() {
    let tests = vec![
        (
            vec![
                Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode::new(5))),
                    })),
                })),
                Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode::new(4))),
                    })),
                })),
                Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode::new(6))),
                })),
            ],
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode {
                        val: 2,
                        next: Some(Box::new(ListNode {
                            val: 3,
                            next: Some(Box::new(ListNode {
                                val: 4,
                                next: Some(Box::new(ListNode {
                                    val: 4,
                                    next: Some(Box::new(ListNode {
                                        val: 5,
                                        next: Some(Box::new(ListNode::new(6))),
                                    })),
                                })),
                            })),
                        })),
                    })),
                })),
            })),
        ),
        (vec![], None),
        (vec![None], None),
    ];

    for (lists, ans) in tests {
        assert_eq!(Solution::merge_k_lists(lists), ans);
    }
}
