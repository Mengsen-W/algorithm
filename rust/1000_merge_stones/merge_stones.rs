/*
 * @Date: 2023-04-04
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-04-04
 * @FilePath: /algorithm/rust/1000_merge_stones/merge_stones.rs
 */

pub fn merge_stones(stones: Vec<i32>, k: i32) -> i32 {
    const INF: i32 = 0x3f3f3f3f;
    let k = k as usize;
    let n = stones.len();
    if (n - 1) % (k - 1) != 0 {
        return -1;
    }

    let mut d = vec![vec![vec![INF; k as usize + 1]; n]; n];
    let mut sum = vec![0; n];

    let mut s = 0;
    for i in 0..n {
        d[i][i][1] = 0;
        s += stones[i];
        sum[i] = s;
    }

    for len in 2..=n {
        for l in 0..n - len + 1 {
            let r = l + len - 1;
            for t in 2..=k {
                for p in (l..r).step_by(k as usize - 1) {
                    d[l][r][t] = d[l][r][t].min(d[l][p][1] + d[p + 1][r][t - 1]);
                }
            }
            d[l][r][1] =
                d[l][r][1].min(d[l][r][k as usize] + sum[r] - if l == 0 { 0 } else { sum[l - 1] });
        }
    }
    d[0][n - 1][1]
}

fn main() {}
