/*
 * @Date: 2021-08-24 09:45:55
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-08-24 10:48:34
 */

struct Solution;

impl Solution {
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        const INF: i32 = 10000 * 101 + 1;
        let n = n as usize;
        let k = k as usize;
        let mut f = vec![vec![INF; n]; k + 2];
        f[0][src as usize] = 0;
        for t in 1..=k + 1 {
            for flight in &flights {
                f[t][flight[1] as usize] =
                    f[t][flight[1] as usize].min(f[t - 1][flight[0] as usize] + flight[2])
            }
        }
        let mut ans = INF;
        for t in 1..=k + 1 {
            ans = ans.min(f[t][dst as usize]);
        }
        match ans == INF {
            true => -1,
            false => ans,
        }
    }
}

fn main() {
    {
        let n = 3;
        let edges = vec![vec![0, 1, 100], vec![1, 2, 100], vec![0, 2, 500]];
        let src = 0;
        let dst = 2;
        let k = 1;
        assert_eq!(Solution::find_cheapest_price(n, edges, src, dst, k), 200);
    }
    {
        let n = 3;
        let edges = vec![vec![0, 1, 100], vec![1, 2, 100], vec![0, 2, 500]];
        let src = 0;
        let dst = 2;
        let k = 0;
        assert_eq!(Solution::find_cheapest_price(n, edges, src, dst, k), 500);
    }
}
