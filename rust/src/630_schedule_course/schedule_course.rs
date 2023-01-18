/*
 * @Date: 2021-12-14 05:54:56
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-12-14 06:10:54
 */

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

fn main() {
    assert_eq!(
        schedule_course(vec![
            vec![100, 200],
            vec![200, 1300],
            vec![1000, 1250],
            vec![2000, 3200]
        ]),
        3
    );
    assert_eq!(schedule_course(vec![vec![1, 2]]), 1);
    assert_eq!(schedule_course(vec![vec![3, 2], vec![4, 3]]), 0);
}
