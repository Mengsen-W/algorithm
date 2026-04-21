/*
 * @Date: 2023-05-16
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-05-16
 * @FilePath: /algorithm/rust/1335_min_difficulty/min_difficulty.rs
 */

pub fn min_difficulty(job_difficulty: Vec<i32>, d: i32) -> i32 {
    let d = d as usize;
    if job_difficulty.len() < d {
        return -1;
    }

    let n = job_difficulty.len();
    let mut dp = vec![vec![0; n]; d];

    dp[0][0] = job_difficulty[0];
    for i in 1..n {
        dp[0][i] = dp[0][i - 1].max(job_difficulty[i]);
    }

    for i in 1..d {
        let mut stk: Vec<(usize, i32)> = vec![];

        for j in i..n {
            let jd = job_difficulty[j];
            let mut cur_min = dp[i - 1][j - 1];

            while !stk.is_empty() {
                let &(idx, val) = stk.last().unwrap();
                if job_difficulty[idx] <= jd {
                    stk.pop();
                    cur_min = cur_min.min(val);
                } else {
                    break;
                }
            }

            dp[i][j] = cur_min + jd;

            if let Some(&(m, _)) = stk.last() {
                dp[i][j] = dp[i][j].min(dp[i][m]);
            }

            stk.push((j, cur_min));
        }
    }
    dp[d - 1][n - 1]
}

fn main() {
    {
        let job_difficulty = vec![6, 5, 4, 3, 2, 1];
        let d = 2;
        let ans = 7;
        assert_eq!(min_difficulty(job_difficulty, d), ans);
    }

    {
        let job_difficulty = vec![9, 9, 9];
        let d = 4;
        let ans = -1;
        assert_eq!(min_difficulty(job_difficulty, d), ans);
    }

    {
        let job_difficulty = vec![1, 1, 1];
        let d = 3;
        let ans = 3;
        assert_eq!(min_difficulty(job_difficulty, d), ans);
    }

    {
        let job_difficulty = vec![7, 1, 7, 1, 7, 1];
        let d = 3;
        let ans = 15;
        assert_eq!(min_difficulty(job_difficulty, d), ans);
    }

    {
        let job_difficulty = vec![11, 111, 22, 222, 33, 333, 44, 444];
        let d = 6;
        let ans = 843;
        assert_eq!(min_difficulty(job_difficulty, d), ans);
    }
}
