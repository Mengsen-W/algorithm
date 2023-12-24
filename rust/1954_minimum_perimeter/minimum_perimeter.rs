/*
 * @Date: 2023-12-24
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-12-24
 * @FilePath: /algorithm/rust/1954_minimum_perimeter/minimum_perimeter.rs
 */

pub struct Solution;

impl Solution {
    pub fn minimum_perimeter(needed_apples: i64) -> i64 {
        let mut left = 1;
        let mut right = 100000;
        let mut ans = 0;

        while left <= right {
            let mid = (left + right) / 2;
            if 2 * mid * (mid + 1) * (mid * 2 + 1) >= needed_apples {
                ans = mid;
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }

        ans * 8
    }
}

fn main() {
    let tests = vec![(1, 8), (13, 16), (1000000000, 5040)];

    for (needed_apples, ans) in tests {
        assert_eq!(Solution::minimum_perimeter(needed_apples), ans);
    }
}
