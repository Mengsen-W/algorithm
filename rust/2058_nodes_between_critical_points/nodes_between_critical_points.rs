struct Solution;

//  Definition for singly-linked list.
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
    pub fn nodes_between_critical_points(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut min_dist = -1;
        let mut max_dist = -1;
        let mut first = -1;
        let mut last = -1;
        let mut pos = 0;
        let mut cur = &head;

        while let Some(node) = cur {
            if node.next.is_none() || node.next.as_ref().unwrap().next.is_none() {
                break;
            }

            // 获取连续的三个节点的值
            let x = node.val;
            let y = node.next.as_ref().unwrap().val;
            let z = node.next.as_ref().unwrap().next.as_ref().unwrap().val;

            // 如果 y 是临界点
            if y > x.max(z) || y < x.min(z) {
                if last != -1 {
                    // 用相邻临界点的距离更新最小值
                    min_dist = if min_dist == -1 {
                        pos - last
                    } else {
                        min_dist.min(pos - last)
                    };
                    // 用到第一个临界点的距离更新最大值
                    max_dist = max_dist.max(pos - first);
                }
                if first == -1 {
                    first = pos;
                }
                // 更新上一个临界点
                last = pos;
            }

            cur = &node.next;
            pos += 1;
        }

        vec![min_dist, max_dist]
    }
}

// 辅助函数：从切片构造链表（头到尾）
fn make_list(vals: &[i32]) -> Option<Box<ListNode>> {
    let mut head = None;
    for &val in vals.iter().rev() {
        head = Some(Box::new(ListNode { val, next: head }));
    }
    head
}

fn main() {
    let tests = vec![
        (make_list(&[3, 1]), vec![-1, -1]),
        (make_list(&[5, 3, 1, 2, 5, 1, 2]), vec![1, 3]),
        (make_list(&[1, 3, 2, 2, 3, 2, 2, 2, 7]), vec![3, 3]),
        (make_list(&[2, 3, 3, 2]), vec![-1, -1]),
    ];

    for (head, ans) in tests {
        assert_eq!(Solution::nodes_between_critical_points(head), ans);
    }
}
