/*
 * @Date: 2021-09-22 09:15:12
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-09-22 09:56:49
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
    pub fn split_list_to_parts(root: Option<Box<ListNode>>, k: i32) -> Vec<Option<Box<ListNode>>> {
        let k = k as usize;
        let mut cnt = 0usize;
        let mut ptr = root.as_ref();

        while let Some(node) = ptr {
            cnt += 1;
            ptr = node.next.as_ref();
        }

        let mut vec = vec![];
        let (a, b) = (cnt / k, cnt % k);
        let mut node = root;

        for i in 0..k {
            let mut num = a;
            if i < b {
                num += 1;
            }
            if num == 0 {
                vec.push(None);
                continue;
            }
            vec.push(node);
            let mut ptr = vec[i].as_mut().unwrap();
            for _ in 0..num - 1 {
                ptr = ptr.next.as_mut().unwrap();
            }
            node = ptr.next.take();
        }
        vec
    }
}

fn main() {
    {
        let head = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode::new(3))),
            })),
        }));
        let k = 5;
        let ans = Solution::split_list_to_parts(head, k);
        println!("{:?}", ans);
    }
    {
        let head = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode {
                            val: 5,
                            next: Some(Box::new(ListNode {
                                val: 6,
                                next: Some(Box::new(ListNode {
                                    val: 7,
                                    next: Some(Box::new(ListNode {
                                        val: 8,
                                        next: Some(Box::new(ListNode {
                                            val: 9,
                                            next: Some(Box::new(ListNode::new(10))),
                                        })),
                                    })),
                                })),
                            })),
                        })),
                    })),
                })),
            })),
        }));
        let k = 5;
        let ans = Solution::split_list_to_parts(head, k);
        println!("{:?}", ans);
    }
}
