/*
 * @Date: 2023-12-08
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-12-08
 * @FilePath: /algorithm/rust/2008_max_taxi_earnings/max_taxi_earnings.rs
 */

struct Solution;

impl Solution {
    pub fn max_taxi_earnings(n: i32, rides: Vec<Vec<i32>>) -> i64 {
        let n = n as usize;
        let mut groups: Vec<Vec<(i32, i32)>> = vec![vec![]; n + 1];
        for r in &rides {
            let start = r[0];
            let end = r[1];
            let tip = r[2];
            groups[end as usize].push((start, end - start + tip));
        }

        let mut f: Vec<i64> = vec![0; n + 1];
        for i in 2..=n {
            f[i] = f[i - 1];
            for &(s, t) in &groups[i] {
                f[i] = f[i].max(f[s as usize] + t as i64);
            }
        }
        f[n]
    }
}

fn main() {
    let tests = vec![
        (5, vec![vec![2, 5, 4], vec![1, 5, 1]], 7),
        (
            30,
            vec![
                vec![1, 6, 1],
                vec![3, 10, 2],
                vec![10, 12, 3],
                vec![11, 12, 2],
                vec![12, 15, 2],
                vec![13, 18, 1],
            ],
            20,
        ),
    ];

    for (n, rides, ans) in tests {
        assert_eq!(Solution::max_taxi_earnings(n, rides), ans);
    }
}
