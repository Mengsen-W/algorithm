/*
 * @Date: 2021-12-14 05:54:56
 * @Author: Mengsen Wang
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-09-11
 */

struct Solution;
impl Solution {
    pub fn schedule_course(courses: Vec<Vec<i32>>) -> i32 {
        use std::collections::BinaryHeap;
        let mut c = courses;
        c.sort_by(|a, b| i32::cmp(&a[1], &b[1]));

        let mut heap = BinaryHeap::new();
        let mut total = 0;

        for cource in c {
            let duration = cource[0];
            let last_day = cource[1];
            if total + duration <= last_day {
                total += duration;
                heap.push(duration);
            } else if !heap.is_empty() && heap.peek().unwrap() > &duration {
                total -= heap.pop().unwrap() - duration;
                heap.push(duration);
            }
        }
        heap.len() as i32
    }
}

fn main() {
    let tests = vec![
        (
            vec![
                vec![100, 200],
                vec![200, 1300],
                vec![1000, 1250],
                vec![2000, 3200],
            ],
            3,
        ),
        (vec![vec![1, 2]], 1),
        (vec![vec![3, 2], vec![4, 3]], 0),
    ];

    for (courses, ans) in tests {
        assert_eq!(schedule_course(courses), ans)
    }
}
