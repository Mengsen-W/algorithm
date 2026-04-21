/*
 * @Date: 2023-10-11
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-10-11
 * @FilePath: /algorithm/rust/2512_top_students/top_students.rs
 */

struct Solution;

use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn top_students(
        positive_feedback: Vec<String>,
        negative_feedback: Vec<String>,
        report: Vec<String>,
        student_id: Vec<i32>,
        k: i32,
    ) -> Vec<i32> {
        let n = student_id.len();
        let ps = positive_feedback.iter().collect::<HashSet<&String>>();
        let ns = negative_feedback.iter().collect::<HashSet<&String>>();
        let mut map = HashMap::new();
        for i in 0..n {
            let id = student_id[i];
            let mut count = 0;
            for s in report[i].split(' ') {
                let s = &s.to_string();
                if ps.contains(s) {
                    count += 3;
                } else if ns.contains(s) {
                    count -= 1;
                }
            }
            map.insert(id, count);
        }
        let mut t = map.into_iter().collect::<Vec<(i32, i32)>>();
        t.sort_by(|a, b| {
            if a.1 == b.1 {
                return a.0.cmp(&b.0);
            }
            b.1.cmp(&a.1)
        });
        t.iter().map(|v| v.0).collect::<Vec<i32>>()[0..k as usize].to_vec()
    }
}

fn main() {
    let tests = vec![
        (
            vec!["smart", "brilliant", "studious"],
            vec!["not"],
            vec!["this student is studious", "the student is smart"],
            vec![1, 2],
            2,
            vec![1, 2],
        ),
        (
            vec!["smart", "brilliant", "studious"],
            vec!["not"],
            vec!["this student is not studious", "the student is smart"],
            vec![1, 2],
            2,
            vec![2, 1],
        ),
    ];

    for (positive_feedback, negative_feedback, report, student_id, k, ans) in tests {
        assert_eq!(
            Solution::top_students(
                positive_feedback.iter().map(|s| s.to_string()).collect(),
                negative_feedback.iter().map(|s| s.to_string()).collect(),
                report.iter().map(|s| s.to_string()).collect(),
                student_id,
                k
            ),
            ans
        );
    }
}
