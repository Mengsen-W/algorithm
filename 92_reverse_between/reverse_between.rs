/*
 * @Date: 2021-03-18 08:44:30
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-03-18 09:28:20
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

fn reverse_between(head: Option<Box<ListNode>>, left: i32, right: i32) -> Option<Box<ListNode>> {
    let mut root = ListNode { val: 0, next: head };
    let mut curr = &mut root;
    let mut cnt = 1;

    while curr.next.is_some() {
        // 找到第left个节点
        if cnt >= left {
            let mut nodes = vec![];
            let mut midl = ListNode {
                val: 0,
                // 通过take获得中间段和尾段
                next: curr.next.take(),
            };
            let mut midl_curr = &mut midl;

            // 找到第right个节点
            while cnt <= right {
                let node = midl_curr.next.take().unwrap();
                nodes.push(node);
                midl_curr = nodes.last_mut().unwrap();
                cnt += 1;
            }

            // 通过take获得尾段
            let rear = midl_curr.next.take();
            let mut midl = ListNode::new(0);
            let mut midl_curr = &mut midl;
            // 翻转中间段
            for node in nodes.into_iter().rev() {
                midl_curr.next = Some(node);
                midl_curr = midl_curr.next.as_mut().unwrap();
            }

            // 连接中间段和尾段
            midl_curr.next = rear;
            // 连接尾段和中间段
            curr.next = midl.next;

            break;
        } else {
            cnt += 1;
            curr = curr.next.as_mut().unwrap();
        }
    }

    root.next
}

fn main() {
    let mut n_1: ListNode = ListNode {
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
    };
    let res = reverse_between(Some(Box::new(n_1)), 2, 4);
    println!("{:?}", res);
}
