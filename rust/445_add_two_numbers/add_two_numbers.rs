/*
 * @Date: 2023-07-03
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-07-03
 * @FilePath: /algorithm/rust/445_add_two_numbers/add_two_numbers.rs
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
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut v1 = Vec::with_capacity(100);
        let mut v2 = Vec::with_capacity(100);
        let mut m_l1 = l1;
        let mut m_l2 = l2;

        let mut answer = Box::new(ListNode::new(0));

        while let Some(t) = m_l1 {
            v1.push(t.val);
            m_l1 = t.next;
        }

        while let Some(t) = m_l2 {
            v2.push(t.val);
            m_l2 = t.next;
        }

        let mut carry = 0;
        while v1.len() != 0 || v2.len() != 0 || carry > 0 {
            let x = if let Some(t) = v1.pop() { t } else { 0 };
            let y = if let Some(t) = v2.pop() { t } else { 0 };
            let tmp = x + y + carry;

            let mut node = ListNode::new(tmp % 10);

            carry = tmp / 10;
            node.next = answer.next.take();
            answer.next = Some(Box::new(node));
        }

        answer.next
    }
}

fn create_list(list: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;
    let mut current = &mut head;
    for x in list {
        *current = Some(Box::new(ListNode::new(x)));
        current = &mut current.as_mut().unwrap().next;
    }
    head
}

fn main() {
    let test_map = vec![
        (vec![7, 2, 3, 4], vec![5, 6, 4], vec![7, 8, 0, 7]),
        (vec![2, 3, 4], vec![5, 6, 4], vec![8, 0, 7]),
        (vec![0], vec![0], vec![0]),
    ];

    for item in test_map {
        assert_eq!(
            Solution::add_two_numbers(create_list(item.0), create_list(item.1)),
            create_list(item.2)
        );
    }
}
