struct Solution;

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
    pub fn modified_list(nums: Vec<i32>, head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        use std::collections::HashSet;
        let is_exist: HashSet<i32> = nums.into_iter().collect();
        let mut sentry = Box::new(ListNode::new(0));
        sentry.next = head;

        let mut p = &mut sentry;
        while let Some(ref mut next_node) = p.next {
            if is_exist.contains(&next_node.val) {
                p.next = next_node.next.take();
            } else {
                p = p.next.as_mut().unwrap();
            }
        }

        sentry.next
    }
}

fn main() {
    let tests = vec![
        (
            vec![1, 2, 3],
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode {
                            val: 4,
                            next: Some(Box::new(ListNode::new(5))),
                        })),
                    })),
                })),
            })),
            Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode::new(5))),
            })),
        ),
        (
            vec![1],
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 1,
                        next: Some(Box::new(ListNode {
                            val: 2,
                            next: Some(Box::new(ListNode {
                                val: 1,
                                next: Some(Box::new(ListNode::new(2))),
                            })),
                        })),
                    })),
                })),
            })),
            Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode::new(2))),
                })),
            })),
        ),
        (
            vec![5],
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
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode::new(4))),
                    })),
                })),
            })),
        ),
    ];

    for (input, head, ans) in tests {
        assert_eq!(Solution::modified_list(input, head), ans);
    }
}
