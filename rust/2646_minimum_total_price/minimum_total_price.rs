/*
 * @Date: 2023-12-06
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-12-07
 * @FilePath: /algorithm/rust/2646_minimum_total_price/minimum_total_price.rs
 */

struct Solution;

impl Solution {
    pub fn minimum_total_price(
        n: i32,
        edges: Vec<Vec<i32>>,
        price: Vec<i32>,
        trips: Vec<Vec<i32>>,
    ) -> i32 {
        let n = n as usize;
        let mut g = vec![vec![]; n];
        for e in &edges {
            let x = e[0] as usize;
            let y = e[1] as usize;
            g[x].push(y);
            g[y].push(x);
        }

        fn dfs(x: usize, fa: usize, cnt: &mut Vec<i32>, g: &Vec<Vec<usize>>, end: usize) -> bool {
            if x == end {
                cnt[x] += 1;
                return true; // 找到 end
            }
            for &y in &g[x] {
                if y != fa && dfs(y, x, cnt, g, end) {
                    cnt[x] += 1; // x 是 end 的祖先节点，也就在路径上
                    return true;
                }
            }
            false // 未找到 end
        }
        let mut cnt = vec![0; n];
        for t in &trips {
            dfs(t[0] as usize, n, &mut cnt, &g, t[1] as usize);
        }

        // 类似 337. 打家劫舍 III
        fn dp(
            x: usize,
            fa: usize,
            price: &Vec<i32>,
            cnt: &Vec<i32>,
            g: &Vec<Vec<usize>>,
        ) -> (i32, i32) {
            let mut not_halve = price[x] * cnt[x]; // x 不变
            let mut halve = not_halve / 2; // x 减半
            for &y in &g[x] {
                if y != fa {
                    let (nh, h) = dp(y, x, price, cnt, g); // 计算 y 不变/减半的最小价值总和
                    not_halve += nh.min(h); // x 不变，那么 y 可以不变或者减半，取这两种情况的最小值
                    halve += nh; // x 减半，那么 y 只能不变
                }
            }
            (not_halve, halve)
        }
        let (nh, h) = dp(0, 0, &price, &cnt, &g);
        nh.min(h)
    }
}

fn main() {
    let tests = vec![
        (
            4,
            vec![vec![0, 1], vec![1, 2], vec![1, 3]],
            vec![2, 2, 10, 6],
            vec![vec![0, 3], vec![2, 1], vec![2, 3]],
            23,
        ),
        (2, vec![vec![0, 1]], vec![2, 2], vec![vec![0, 0]], 1),
    ];

    for (n, edges, price, trips, ans) in tests {
        assert_eq!(Solution::minimum_total_price(n, edges, price, trips), ans);
    }
}
