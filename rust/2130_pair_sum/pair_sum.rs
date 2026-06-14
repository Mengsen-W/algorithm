use std::boxed::Box;
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
    pub fn pair_sum(head: Option<Box<ListNode>>) -> i32 {
        let slow = &head as *const Option<Box<ListNode>>;
        let mut fast = &head;

        unsafe {
            let mut slow_ptr = slow as *mut Option<Box<ListNode>>;
            while let Some(fast_node) = fast {
                if let Some(fast_next) = &fast_node.next {
                    fast = &fast_next.next;
                    if let Some(slow_node) = &*(slow_ptr) {
                        slow_ptr = &(slow_node.next) as *const _ as *mut Option<Box<ListNode>>;
                    }
                } else {
                    break;
                }
            }

            let mut second_half = std::mem::replace(&mut *slow_ptr, None);
            let mut prev = None;

            while let Some(mut node) = second_half {
                let next = node.next.take();
                node.next = prev;
                prev = Some(node);
                second_half = next;
            }

            let mut first = &head;
            let mut second = &prev;
            let mut max_sum = 0;

            while let (Some(f), Some(s)) = (first, second) {
                max_sum = max_sum.max(f.val + s.val);
                first = &f.next;
                second = &s.next;
            }

            max_sum
        }
    }
}

fn main() {
    let tests = vec![
        (
            Some(Box::new(ListNode {
                val: 5,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode {
                        val: 2,
                        next: Some(Box::new(ListNode::new(1))),
                    })),
                })),
            })),
            6,
        ),
        (
            Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 2,
                        next: Some(Box::new(ListNode::new(3))),
                    })),
                })),
            })),
            7,
        ),
        (
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode::new(100000))),
            })),
            100001,
        ),
    ];

    for (head, expected) in tests {
        assert_eq!(Solution::pair_sum(head), expected);
    }
}
