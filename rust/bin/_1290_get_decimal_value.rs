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
    pub fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
        let mut ans = 0;
        let mut cur = &head;
        while let Some(node) = cur {
            ans = ans * 2 + node.val;
            cur = &node.next;
        }
        ans
    }
}

fn main() {
    let tests = vec![
        (
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 0,
                    next: Some(Box::new(ListNode::new(1))),
                })),
            })),
            5,
        ),
        (
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 0,
                    next: Some(Box::new(ListNode {
                        val: 0,
                        next: Some(Box::new(ListNode {
                            val: 1,
                            next: Some(Box::new(ListNode {
                                val: 0,
                                next: Some(Box::new(ListNode {
                                    val: 0,
                                    next: Some(Box::new(ListNode {
                                        val: 1,
                                        next: Some(Box::new(ListNode {
                                            val: 1,
                                            next: Some(Box::new(ListNode {
                                                val: 1,
                                                next: Some(Box::new(ListNode {
                                                    val: 0,
                                                    next: Some(Box::new(ListNode {
                                                        val: 0,
                                                        next: Some(Box::new(ListNode {
                                                            val: 0,
                                                            next: Some(Box::new(ListNode {
                                                                val: 0,
                                                                next: Some(Box::new(ListNode {
                                                                    val: 0,
                                                                    next: Some(Box::new(
                                                                        ListNode::new(0),
                                                                    )),
                                                                })),
                                                            })),
                                                        })),
                                                    })),
                                                })),
                                            })),
                                        })),
                                    })),
                                })),
                            })),
                        })),
                    })),
                })),
            })),
            18880,
        ),
        (Some(Box::new(ListNode { val: 0, next: None })), 0),
        (Some(Box::new(ListNode { val: 1, next: None })), 1),
        (
            Some(Box::new(ListNode {
                val: 0,
                next: Some(Box::new(ListNode::new(0))),
            })),
            0,
        ),
    ];

    for (test, expected) in tests {
        assert_eq!(Solution::get_decimal_value(test), expected);
    }
}
