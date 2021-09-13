/*
 * @Date: 2021-09-13 08:20:54
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-09-13 08:37:09
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
    {
        let points = vec![vec![0, 0], vec![1, 0], vec![2, 0]];
        let ans = 2;
        assert_eq!(Solution::number_of_boomerangs(points), ans);
    }
    {
        let points = vec![vec![1, 1], vec![2, 2], vec![3, 3]];
        let ans = 2;
        assert_eq!(Solution::number_of_boomerangs(points), ans);
    }
    {
        let points = vec![vec![1, 1]];
        let ans = 0;
        assert_eq!(Solution::number_of_boomerangs(points), ans);
    }
}
