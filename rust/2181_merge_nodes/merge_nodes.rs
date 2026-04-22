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
    pub fn merge_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut tail = &mut dummy;
        let mut total = 0;
        let mut cur = head.unwrap().next;
        while let Some(mut node) = cur {
            if node.val == 0 {
                tail.next = Some(Box::new(ListNode::new(total)));
                tail = tail.next.as_mut().unwrap();
                total = 0;
            } else {
                total += node.val;
            }
            cur = node.next.take();
        }

        dummy.next
    }
}

fn main() {
    let tests = vec![
        (
            Some(Box::new(ListNode {
                val: 0,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 1,
                        next: Some(Box::new(ListNode {
                            val: 0,
                            next: Some(Box::new(ListNode {
                                val: 4,
                                next: Some(Box::new(ListNode {
                                    val: 5,
                                    next: Some(Box::new(ListNode {
                                        val: 2,
                                        next: Some(Box::new(ListNode::new(0))),
                                    })),
                                })),
                            })),
                        })),
                    })),
                })),
            })),
            Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode::new(11))),
            })),
        ),
        (
            Some(Box::new(ListNode {
                val: 0,
                next: Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode {
                        val: 0,
                        next: Some(Box::new(ListNode {
                            val: 3,
                            next: Some(Box::new(ListNode {
                                val: 0,
                                next: Some(Box::new(ListNode {
                                    val: 2,
                                    next: Some(Box::new(ListNode {
                                        val: 2,
                                        next: Some(Box::new(ListNode::new(0))),
                                    })),
                                })),
                            })),
                        })),
                    })),
                })),
            })),
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode::new(4))),
                })),
            })),
        ),
    ];

    for (head, ans) in tests {
        assert_eq!(Solution::merge_nodes(head), ans);
    }
}
