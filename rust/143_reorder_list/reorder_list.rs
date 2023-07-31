/*
 * @Date: 2023-07-31
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-07-31
 * @FilePath: /algorithm/rust/143_reorder_list/reorder_list.rs
 */

struct Solution;

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
impl Solution {
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        let mut nodes: Vec<Option<Box<ListNode>>> = Vec::new();
        let mut curr = head.take();

        while let Some(mut node) = curr {
            curr = node.next.take();
            nodes.push(Some(node));
        }

        let len = nodes.len();
        let mid = len / 2;

        let mut dummy = Box::new(ListNode::new(i32::default()));
        let mut curr = &mut dummy;
        for i in 0..mid {
            curr.next = nodes[i].take();
            curr = curr.next.as_mut().unwrap();
            curr.next = nodes[len - i - 1].take();
            curr = curr.next.as_mut().unwrap();
        }

        if mid < len - mid {
            curr.next = nodes[mid].take();
        }

        *head = dummy.next;
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
                        next: Some(Box::new(ListNode { val: 4, next: None })),
                    })),
                })),
            })),
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode {
                        val: 2,
                        next: Some(Box::new(ListNode { val: 3, next: None })),
                    })),
                })),
            })),
        ),
        (
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode {
                            val: 4,
                            next: Some(Box::new(ListNode { val: 5, next: None })),
                        })),
                    })),
                })),
            })),
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 5,
                    next: Some(Box::new(ListNode {
                        val: 2,
                        next: Some(Box::new(ListNode {
                            val: 4,
                            next: Some(Box::new(ListNode { val: 3, next: None })),
                        })),
                    })),
                })),
            })),
        ),
    ];

    for (input, output) in tests {
        let mut input = input;
        Solution::reorder_list(&mut input);
        assert_eq!(input, output);
    }
}
