/*
 * @Date: 2024-01-06
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-01-06
 * @FilePath: /algorithm/rust/2807_insert_greatest_common_divisors/insert_greatest_common_divisors.rs
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
    pub fn insert_greatest_common_divisors(
        mut head: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut cur = &mut head;
        while cur.as_ref().unwrap().next.is_some() {
            let x = cur.as_mut().unwrap();
            let next = x.next.take();
            x.next = Some(Box::new(ListNode {
                val: Self::gcd(x.val, next.as_ref().unwrap().val),
                next,
            }));
            cur = &mut cur.as_mut().unwrap().next.as_mut().unwrap().next;
        }
        head
    }

    fn gcd(mut a: i32, mut b: i32) -> i32 {
        while a != 0 {
            (a, b) = (b % a, a);
        }
        b
    }
}

fn main() {
    let tests = vec![
        (
            Some(Box::new(ListNode {
                val: 18,
                next: Some(Box::new(ListNode {
                    val: 6,
                    next: Some(Box::new(ListNode {
                        val: 10,
                        next: Some(Box::new(ListNode::new(3))),
                    })),
                })),
            })),
            Some(Box::new(ListNode {
                val: 18,
                next: Some(Box::new(ListNode {
                    val: 6,
                    next: Some(Box::new(ListNode {
                        val: 6,
                        next: Some(Box::new(ListNode {
                            val: 2,
                            next: Some(Box::new(ListNode {
                                val: 10,
                                next: Some(Box::new(ListNode {
                                    val: 1,
                                    next: Some(Box::new(ListNode::new(3))),
                                })),
                            })),
                        })),
                    })),
                })),
            })),
        ),
        (
            Some(Box::new(ListNode::new(7))),
            Some(Box::new(ListNode::new(7))),
        ),
    ];

    for (head, ans) in tests {
        assert_eq!(Solution::insert_greatest_common_divisors(head), ans);
    }
}
