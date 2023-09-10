/*
 * @Date: 2023-09-10
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-09-10
 * @FilePath: /algorithm/rust/210_find_order/find_order.rs
 */

struct Solution;
impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        use std::collections::VecDeque;
        let n = num_courses as usize;

        // 1.
        let mut in_degree = vec![0; n];
        let mut graph = vec![vec![]; n];

        for v in prerequisites.into_iter() {
            let (num1, num2) = (v[0] as usize, v[1] as usize);
            in_degree[num1] += 1;
            graph[num2].push(num1);
        }

        // 2.
        let mut result = vec![];
        let mut queue = VecDeque::new();
        for (i, &val) in in_degree.iter().enumerate() {
            if val == 0 {
                queue.push_back(i);
                result.push(i as i32);
            }
        }

        // 3.
        while let Some(cur_course) = queue.pop_front() {
            for &next_course in graph[cur_course].iter() {
                in_degree[next_course] -= 1;
                if in_degree[next_course] == 0 {
                    queue.push_back(next_course);
                    result.push(next_course as i32);
                }
            }
        }

        // 4.
        if result.len() == n {
            result
        } else {
            vec![]
        }
    }
}

fn main() {
    let tests = vec![(2, vec![vec![1, 0]], vec![0, 1]), (1, vec![], vec![0])];

    for (next_course, prerequisites, ans) in tests {
        assert_eq!(Solution::find_order(next_course, prerequisites), ans);
    }
}
