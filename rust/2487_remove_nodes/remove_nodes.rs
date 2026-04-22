/*
 * @Date: 2024-01-03
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-01-03
 * @FilePath: /algorithm/rust/2487_remove_nodes/remove_nodes.rs
 */

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

struct Solution;

impl Solution {
    pub fn remove_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if let Some(mut node) = head {
            let next = node.next.take();
            node.next = Solution::remove_nodes(next);
            if let Some(next_node) = node.next.as_ref() {
                if node.val < next_node.val {
                    return node.next;
                }
            }
            Some(node)
        } else {
            None
        }
    }
}

fn main() {
    let tests = vec![
        (
            Some(Box::new(ListNode {
                val: 5,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 13,
                        next: Some(Box::new(ListNode {
                            val: 3,
                            next: Some(Box::new(ListNode { val: 8, next: None })),
                        })),
                    })),
                })),
            })),
            Some(Box::new(ListNode {
                val: 13,
                next: Some(Box::new(ListNode { val: 8, next: None })),
            })),
        ),
        (
            Some(Box::new(ListNode {
                val: 5,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 13,
                        next: Some(Box::new(ListNode {
                            val: 3,
                            next: Some(Box::new(ListNode { val: 8, next: None })),
                        })),
                    })),
                })),
            })),
            Some(Box::new(ListNode {
                val: 13,
                next: Some(Box::new(ListNode { val: 8, next: None })),
            })),
        ),
    ];

    for (head, ans) in tests {
        assert_eq!(Solution::remove_nodes(head), ans);
    }
}
