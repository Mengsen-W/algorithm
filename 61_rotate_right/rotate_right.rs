/*
 * @Date: 2021-03-27 07:54:39
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-03-27 08:28:41
 */

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

fn rotate_right(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    debug_assert!(k >= 0);

    if head.is_none() {
        return None;
    }

    if k == 0 {
        return head;
    }

    let mut len = 0;
    let mut ptr = &head;
    while let Some(ref node) = ptr {
        len += 1;
        ptr = &node.next;
    }

    let step = k % len;

    if step == 0 || len == 1 {
        return head;
    }

    let mut ptr = &mut head;
    for _ in 1..(len - step) {
        ptr = &mut ptr.as_mut().unwrap().next;
    }

    // 找到断开点，断开，后面的就是新 head
    let mut new_head = ptr.as_mut().unwrap().next.take();

    // 找到 tail
    let mut tail = &mut new_head;
    while tail.is_some() && tail.as_ref().unwrap().next.is_some() {
        tail = &mut tail.as_mut().unwrap().next;
    }

    // 把原来的 head 接到 tail 后
    tail.as_mut().unwrap().next = head;

    new_head
}

fn main() {
    let head = ListNode {
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

    let cur = rotate_right(Some(Box::new(head)), 2);
    println!("{:?}", cur);

    let head = ListNode {
        val: 0,
        next: Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode { val: 2, next: None })),
        })),
    };

    let cur = rotate_right(Some(Box::new(head)), 4);
    println!("{:?}", cur);
}
