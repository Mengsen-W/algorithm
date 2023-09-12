/*
 * @Date: 2023-09-12
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-09-12
 * @FilePath: /algorithm/rust/1462_check_if_prerequisite/check_if_prerequisite.rs
 */

struct Solution;

impl Solution {
    pub fn check_if_prerequisite(
        num_courses: i32,
        prerequisites: Vec<Vec<i32>>,
        queries: Vec<Vec<i32>>,
    ) -> Vec<bool> {
        let len = num_courses as usize;
        let mut states = vec![vec![false; len]; len];

        prerequisites
            .into_iter()
            .map(|v| (v[0] as usize, v[1] as usize))
            .for_each(|(from, to)| {
                states[from][to] = true;
            });

        for j in 0..len {
            for i in 0..len {
                for k in 0..len {
                    if states[i][j] && states[j][k] {
                        states[i][k] = true;
                    }
                }
            }
        }

        queries
            .into_iter()
            .map(|v| (v[0] as usize, v[1] as usize))
            .map(|(from, to)| states[from][to])
            .collect()
    }
}

fn main() {
    let tests = vec![
        (
            2,
            vec![vec![1, 0]],
            vec![vec![0, 1], vec![1, 0]],
            vec![false, true],
        ),
        (2, vec![], vec![vec![1, 0], vec![0, 1]], vec![false, false]),
        (
            3,
            vec![vec![1, 2], vec![1, 0], vec![2, 0]],
            vec![vec![1, 0], vec![1, 2]],
            vec![true, true],
        ),
    ];

    for (num_courses, prerequisites, queries, result) in tests {
        assert_eq!(
            Solution::check_if_prerequisite(num_courses, prerequisites, queries),
            result
        );
    }
}
