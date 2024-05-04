/*
 * @Date: 2022-10-22
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-05-04
 * @FilePath: /algorithm/rust/1235_job_scheduling/job_scheduling.rs
 */

struct Solution;

impl Solution {
    pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
        let n = start_time.len();
        let mut job = vec![];

        for i in 0..n {
            job.push((start_time[i], end_time[i], profit[i]));
        }
        job.sort_by_key(|&(_, b, _)| b);

        let mut dp = vec![0; n + 1];
        for i in 1..n + 1 {
            let mut profit = job[i - 1].2;
            match job[0..i - 1].binary_search_by_key(&job[i - 1].0, |&(_, b, _)| b) {
                Ok(mut j) => {
                    while j + 1 < n && job[j + 1].1 == job[i - 1].0 {
                        j += 1;
                    }
                    profit += dp[j + 1];
                }
                Err(j) => {
                    profit += dp[j];
                }
            }
            dp[i] = profit.max(dp[i - 1]);
        }

        dp[n]
    }
}

fn main() {
    let tests = vec![
        (
            vec![1, 2, 3, 3],
            vec![3, 4, 5, 6],
            vec![50, 10, 40, 70],
            120,
        ),
        (
            vec![1, 2, 3, 4, 6],
            vec![3, 5, 10, 6, 9],
            vec![20, 20, 100, 70, 60],
            150,
        ),
        (vec![1, 1, 1], vec![2, 3, 4], vec![5, 6, 4], 6),
    ];

    for (start_time, end_time, profit, ans) in tests {
        assert_eq!(Solution::job_scheduling(start_time, end_time, profit), ans);
    }
}
