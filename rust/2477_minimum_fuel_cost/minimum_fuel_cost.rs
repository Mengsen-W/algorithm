/*
 * @Date: 2023-12-05
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-12-06
 * @FilePath: /algorithm/rust/2477_minimum_fuel_cost/minimum_fuel_cost.rs
 */

struct Solution;

impl Solution {
    pub fn minimum_fuel_cost(roads: Vec<Vec<i32>>, seats: i32) -> i64 {
        let n = roads.len() + 1;
        let mut g: Vec<Vec<usize>> = vec![vec![]; n];
        for road in roads.iter() {
            let a = road[0] as usize;
            let b = road[1] as usize;
            g[a].push(b);
            g[b].push(a);
        }
        let mut ans = 0;
        fn dfs(a: usize, fa: i32, g: &Vec<Vec<usize>>, ans: &mut i64, seats: i32) -> i32 {
            let mut sz = 1;
            for &b in g[a].iter() {
                if (b as i32) != fa {
                    let t = dfs(b, a as i32, g, ans, seats);
                    *ans += ((t + seats - 1) / seats) as i64;
                    sz += t;
                }
            }
            sz
        }
        dfs(0, -1, &g, &mut ans, seats);
        ans
    }
}

fn main() {
    let tests = vec![
        (vec![vec![0, 1], vec![0, 2], vec![0, 3]], 5, 3),
        (
            vec![
                vec![3, 1],
                vec![3, 2],
                vec![1, 0],
                vec![0, 4],
                vec![0, 5],
                vec![4, 6],
            ],
            2,
            7,
        ),
        (vec![], 1, 0),
    ];

    for (roads, seats, ans) in tests {
        assert_eq!(Solution::minimum_fuel_cost(roads, seats), ans);
    }
}
