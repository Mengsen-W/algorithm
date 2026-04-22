/*
 * @Date: 2024-03-15
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-03-15
 * @FilePath: /algorithm/rust/2312_selling_wood/selling_wood.rs
 */

struct Solution;

impl Solution {
    pub fn selling_wood(m: i32, n: i32, prices: Vec<Vec<i32>>) -> i64 {
        let m = m as usize;
        let n = n as usize;
        let mut f = vec![vec![0; n + 1]; m + 1];
        for p in &prices {
            f[p[0] as usize][p[1] as usize] = p[2] as i64;
        }
        for i in 1..=m {
            for j in 1..=n {
                for k in 1..j {
                    // 垂直切割，枚举宽度 k
                    f[i][j] = f[i][j].max(f[i][k] + f[i][j - k]);
                }
                for k in 1..i {
                    // 水平切割，枚举高度 k
                    f[i][j] = f[i][j].max(f[k][j] + f[i - k][j]);
                }
            }
        }
        f[m][n]
    }
}

fn main() {
    let tests = vec![
        (3, 5, vec![vec![1, 4, 2], vec![2, 2, 7], vec![2, 1, 3]], 19),
        (4, 6, vec![vec![3, 2, 10], vec![1, 4, 2], vec![4, 1, 3]], 32),
    ];

    for (m, n, prices, ans) in tests {
        assert_eq!(Solution::selling_wood(m, n, prices), ans);
    }
}
