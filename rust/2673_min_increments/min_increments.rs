/*
 * @Date: 2024-02-28
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-02-28
 * @FilePath: /algorithm/rust/2673_min_increments/min_increments.rs
 */

struct Solution;

impl Solution {
    pub fn min_increments(n: i32, cost: Vec<i32>) -> i32 {
        let mut cost = cost;
        let mut ans = 0;
        for i in (1..=n as usize / 2).rev() {
            // 从最后一个非叶节点开始算
            ans += (cost[i * 2 - 1] - cost[i * 2]).abs(); // 两个子节点变成一样的
            cost[i - 1] += cost[i * 2 - 1].max(cost[i * 2]); // 累加路径和
        }
        ans
    }
}

fn main() {
    let tests = vec![(7, vec![1, 5, 2, 2, 3, 3, 1], 6), (3, vec![5, 3, 3], 0)];

    for (n, cost, ans) in tests {
        assert_eq!(Solution::min_increments(n, cost), ans);
    }
}
