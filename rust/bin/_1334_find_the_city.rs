/*
 * @Date: 2023-11-14
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-11-14
 * @FilePath: /algorithm/rust/1334_find_the_city/find_the_city.rs
 */

struct Solution;

impl Solution {
    pub fn find_the_city(n: i32, edges: Vec<Vec<i32>>, distance_threshold: i32) -> i32 {
        let n = n as usize;
        let mut w = vec![vec![i32::MAX / 2; n]; n]; // 除 2 防止加法溢出
        for e in &edges {
            let x = e[0] as usize;
            let y = e[1] as usize;
            let wt = e[2];
            w[x][y] = wt;
            w[y][x] = wt;
        }

        let mut f = w;
        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    f[i][j] = f[i][j].min(f[i][k] + f[k][j]);
                }
            }
        }

        let mut ans = 0;
        let mut min_cnt = n;
        for i in 0..n {
            let mut cnt = 0;
            for j in 0..n {
                if j != i && f[i][j] <= distance_threshold {
                    cnt += 1;
                }
            }
            if cnt <= min_cnt {
                // 相等时取最大的 i
                min_cnt = cnt;
                ans = i;
            }
        }
        ans as i32
    }
}

fn main() {
    let tests = vec![
        (
            4,
            vec![vec![0, 1, 3], vec![1, 2, 1], vec![1, 3, 4], vec![2, 3, 1]],
            4,
            3,
        ),
        (
            5,
            vec![
                vec![0, 1, 2],
                vec![0, 4, 8],
                vec![1, 2, 3],
                vec![1, 4, 2],
                vec![2, 3, 1],
                vec![3, 4, 1],
            ],
            2,
            0,
        ),
    ];

    for (n, edges, distance_threshold, ans) in tests {
        assert_eq!(Solution::find_the_city(n, edges, distance_threshold), ans);
    }
}
