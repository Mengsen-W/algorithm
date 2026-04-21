/*
 * @Date: 2023-06-16
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-06-16
 * @FilePath: /algorithm/rust/1494_min_number_of_semesters/min_number_of_semesters.rs
 */

struct Solution;

impl Solution {
    pub fn min_number_of_semesters(n: i32, relations: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut dp = vec![i32::MAX; 1 << n];
        let mut need = vec![0; 1 << n];

        for r in relations {
            need[1 << (r[1] as usize - 1)] |= 1 << (r[0] - 1)
        }

        dp[0] = 0;

        for i in 1..(1 << n) {
            need[i as usize] = need[(i & (i - 1)) as usize] | need[(i & (-i)) as usize];

            if (need[i as usize] | i) != i {
                continue;
            }

            let v = i as i32 ^ need[i as usize];

            if Self::bit_cnt(v) <= k {
                dp[i as usize] = dp[i as usize].min(dp[(i ^ v) as usize] + 1);
            } else {
                let mut u = v;

                while u != 0 {
                    if Self::bit_cnt(u) <= k {
                        dp[i as usize] = dp[i as usize].min(dp[(i ^ u) as usize] + 1);
                    }

                    u = (u - 1) & v;
                }
            }
        }

        dp[(1 << n) - 1]
    }

    fn bit_cnt(mut v: i32) -> i32 {
        let mut cnt = 0;

        while v != 0 {
            cnt += 1;
            v &= v - 1;
        }

        return cnt;
    }
}

fn main() {
    {
        let n = 4;
        let relations = vec![vec![2, 1], vec![3, 1], vec![1, 4]];
        let k = 2;
        let ans = 3;
        assert_eq!(Solution::min_number_of_semesters(n, relations, k), ans);
    }

    {
        let n = 5;
        let relations = vec![vec![2, 1], vec![3, 1], vec![4, 1], vec![1, 5]];
        let k = 2;
        let ans = 4;
        assert_eq!(Solution::min_number_of_semesters(n, relations, k), ans);
    }

    {
        let n = 11;
        let relations = vec![];
        let k = 2;
        let ans = 6;
        assert_eq!(Solution::min_number_of_semesters(n, relations, k), ans);
    }
}
