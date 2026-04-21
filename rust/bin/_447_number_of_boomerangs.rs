/*
 * @Date: 2021-09-13 08:20:54
 * @Author: Mengsen Wang
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-01-08
 */

struct Solution;

impl Solution {
    pub fn number_of_boomerangs(points: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        for p in &points {
            let mut cnt: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();
            for q in &points {
                let dis = (p[0] - q[0]) * (p[0] - q[0]) + (p[1] - q[1]) * (p[1] - q[1]);
                *cnt.entry(dis).or_insert(0) += 1;
            }
            for (_, value) in cnt {
                ans += value * (value - 1);
            }
        }
        ans
    }
}

fn main() {
    let tests = vec![
        (vec![vec![0, 0], vec![1, 0], vec![2, 0]], 2),
        (vec![vec![1, 1], vec![2, 2], vec![3, 3]], 2),
        (vec![vec![1, 1]], 0),
    ];

    for (points, ans) in tests {
        assert_eq!(Solution::number_of_boomerangs(points), ans);
    }
}
