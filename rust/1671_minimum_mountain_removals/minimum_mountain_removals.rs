/*
 * @Date: 2023-12-22
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-12-22
 * @FilePath: /algorithm/rust/1671_minimum_mountain_removals/minimum_mountain_removals.rs
 */

struct Solution;

impl Solution {
    pub fn minimum_mountain_removals(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut suf = vec![0; n];
        let mut g = Vec::new();
        for (i, &x) in nums.iter().enumerate().rev() {
            let j = g.partition_point(|&v| v < x);
            if j == g.len() {
                g.push(x);
            } else {
                g[j] = x;
            }
            suf[i] = j + 1;
        }

        let mut mx = 0;
        g.clear();
        for (i, &x) in nums.iter().enumerate() {
            let j = g.partition_point(|&v| v < x);
            if j == g.len() {
                g.push(x);
            } else {
                g[j] = x;
            }
            let pre = j + 1;
            if pre >= 2 && suf[i] >= 2 {
                mx = mx.max(pre + suf[i] - 1);
            }
        }
        (n - mx) as i32
    }
}

fn main() {
    let tests = vec![(vec![1, 3, 1], 0), (vec![2, 1, 1, 5, 6, 2, 3, 1], 3)];

    for (nums, ans) in tests {
        assert_eq!(Solution::minimum_mountain_removals(nums), ans);
    }
}
