/*
 * @Date: 2022-10-22
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-10-22
 * @FilePath: /algorithm/1235_job_scheduling/job_scheduling.rs
 */

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

fn main() {
    {
        let start_time = vec![1, 2, 3, 3];
        let end_time = vec![3, 4, 5, 6];
        let profit = vec![50, 10, 40, 70];
        let ans = 120;
        assert_eq!(job_scheduling(start_time, end_time, profit), ans);
    }

    {
        let start_time = vec![1, 2, 3, 4, 6];
        let end_time = vec![3, 5, 10, 6, 9];
        let profit = vec![20, 20, 100, 70, 60];
        let ans = 150;
        assert_eq!(job_scheduling(start_time, end_time, profit), ans);
    }

    {
        let start_time = vec![1, 1, 1];
        let end_time = vec![2, 3, 4];
        let profit = vec![5, 6, 4];
        let ans = 6;
        assert_eq!(job_scheduling(start_time, end_time, profit), ans);
    }
}
