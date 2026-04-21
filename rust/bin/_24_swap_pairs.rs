/*
 * @Date: 2023-08-06
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-08-06
 * @FilePath: /algorithm/rust/24_swap_pairs/swap_pairs.rs
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
impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match head {
            Some(mut head) => match head.next.take() {
                Some(mut next) => {
                    head.next = Self::swap_pairs(next.next.take());
                    next.next = Some(head);
                    Some(next)
                }
                None => Some(head),
            },
            None => None,
        }
    }
}

fn main() {
    let tests = vec![
        (
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode::new(4))),
                    })),
                })),
            })),
            Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode::new(3))),
                    })),
                })),
            })),
        ),
        (None, None),
        (
            Some(Box::new(ListNode::new(1))),
            Some(Box::new(ListNode::new(1))),
        ),
    ];

    for (head, ans) in tests {
        assert_eq!(Solution::swap_pairs(head), ans)
    }
}
