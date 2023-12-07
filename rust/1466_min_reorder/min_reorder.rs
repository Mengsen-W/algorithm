/*
 * @Date: 2023-12-07
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-12-07
 * @FilePath: /algorithm/rust/1466_min_reorder/min_reorder.rs
 */

struct Solution;

impl Solution {
    pub fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        let mut g: Vec<Vec<(i32, i32)>> = vec![vec![]; n as usize];
        for e in connections.iter() {
            let a = e[0] as usize;
            let b = e[1] as usize;
            g[a].push((b as i32, 1));
            g[b].push((a as i32, 0));
        }
        fn dfs(a: usize, fa: i32, g: &Vec<Vec<(i32, i32)>>) -> i32 {
            let mut ans = 0;
            for &(b, c) in g[a].iter() {
                if b != fa {
                    ans += c + dfs(b as usize, a as i32, g);
                }
            }
            ans
        }
        dfs(0, -1, &g)
    }
}

fn main() {
    let tests = vec![
        (
            6,
            vec![vec![0, 1], vec![1, 3], vec![2, 3], vec![4, 0], vec![4, 5]],
            3,
        ),
        (5, vec![vec![1, 0], vec![1, 2], vec![3, 2], vec![3, 4]], 2),
        (3, vec![vec![1, 0], vec![2, 0]], 0),
    ];

    for (n, connections, ans) in tests {
        assert_eq!(Solution::min_reorder(n, connections), ans);
    }
}
