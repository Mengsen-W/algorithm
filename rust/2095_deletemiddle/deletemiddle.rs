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
    pub fn delete_middle(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() || head.as_ref()?.next.is_none() {
            return None;
        }

        let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
        let mut slow = &mut dummy as *mut Option<Box<ListNode>>;
        let mut fast = dummy.as_ref().unwrap().next.as_ref();
        while fast.is_some() && fast.unwrap().next.is_some() {
            fast = fast.unwrap().next.as_ref().unwrap().next.as_ref();
            unsafe {
                slow = &mut (*slow).as_mut().unwrap().next as *mut Option<Box<ListNode>>;
            }
        }

        unsafe {
            let next_next = (*slow).as_mut().unwrap().next.as_mut().unwrap().next.take();
            (*slow).as_mut().unwrap().next = next_next;
        }

        dummy.unwrap().next
    }
}

fn main() {
    let tests = vec![
        (
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode {
                            val: 7,
                            next: Some(Box::new(ListNode {
                                val: 1,
                                next: Some(Box::new(ListNode {
                                    val: 2,
                                    next: Some(Box::new(ListNode::new(6))),
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
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode {
                            val: 1,
                            next: Some(Box::new(ListNode {
                                val: 2,
                                next: Some(Box::new(ListNode::new(6))),
                            })),
                        })),
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
                        next: Some(Box::new(ListNode::new(4))),
                    })),
                })),
            })),
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode::new(4))),
                })),
            })),
        ),
        (
            Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode::new(1))),
            })),
            Some(Box::new(ListNode::new(2))),
        ),
    ];

    for (head, expected) in tests {
        assert_eq!(Solution::delete_middle(head), expected);
    }
}
