/*
 * @Date: 2023-09-09
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-09-09
 * @FilePath: /algorithm/rust/207_can_finish/can_finish.rs
 */

struct Solution;

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        use std::collections::HashMap;
        use std::collections::VecDeque;
        let mut income = vec![0; num_courses as usize];
        let mut map = HashMap::new();
        prerequisites.iter().for_each(|x| {
            let (a, b) = (x[0] as usize, x[1] as usize);
            income[a] += 1;
            map.entry(b).or_insert(vec![]).push(a);
        });
        let mut courses = VecDeque::new();
        for (i, &n) in income.iter().enumerate() {
            if n == 0 {
                courses.push_back(i);
            }
        }

        let mut ans = 0;
        while !courses.is_empty() {
            ans += 1;
            let course = courses.pop_front().unwrap();
            if let Some(next) = map.get(&course) {
                for &n in next {
                    income[n] -= 1;
                    if income[n] == 0 {
                        courses.push_back(n);
                    }
                }
            }
        }
        ans == num_courses
    }
}

fn main() {
    let tests = vec![
        (2, vec![vec![1, 0]], true),
        (2, vec![vec![1, 0], vec![0, 1]], false),
    ];

    for (num_courses, prerequisites, ans) in tests {
        assert_eq!(Solution::can_finish(num_courses, prerequisites), ans);
    }
}
