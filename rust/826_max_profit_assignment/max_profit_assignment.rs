/*
 * @Date: 2024-05-17
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-05-17
 * @FilePath: /algorithm/rust/826_max_profit_assignment/max_profit_assignment.rs
 */

struct Solution;

impl Solution {
    pub fn max_profit_assignment(difficulty: Vec<i32>, profit: Vec<i32>, worker: Vec<i32>) -> i32 {
        let mut jobs: Vec<(i32, i32)> = difficulty.into_iter().zip(profit).collect();
        jobs.sort_by_key(|&x| x.0);

        let mut worker = worker;
        worker.sort();

        let mut res = 0;
        let mut i = 0;
        let mut best = 0;
        for w in worker {
            while i < jobs.len() && w >= jobs[i].0 {
                best = std::cmp::max(best, jobs[i].1);
                i += 1;
            }
            res += best;
        }
        res
    }
}

fn main() {
    let tests = vec![
        (
            vec![2, 4, 6, 8, 10],
            vec![10, 20, 30, 40, 50],
            vec![4, 5, 6, 7],
            100,
        ),
        (vec![85, 47, 57], vec![24, 66, 99], vec![40, 25, 25], 0),
    ];

    for (difficulty, profit, worker, ans) in tests {
        assert_eq!(
            Solution::max_profit_assignment(difficulty, profit, worker),
            ans
        );
    }
}
