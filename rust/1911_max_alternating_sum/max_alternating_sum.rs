/*
 * @Date: 2023-07-11
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-07-11
 * @FilePath: /algorithm/rust/1911_max_alternating_sum/max_alternating_sum.rs
 */

struct Solution;
impl Solution {
    pub fn max_alternating_sum(nums: Vec<i32>) -> i64 {
        nums.into_iter()
            .fold((0, 0), |(sum_next_odd, sum_next_even), n| {
                (
                    (sum_next_even + n as i64).max(sum_next_odd),
                    (sum_next_odd - n as i64).max(sum_next_even),
                )
            })
            .0
    }
}

fn main() {
    let tests = vec![
        (vec![4, 2, 5, 3, 7], 11),
        (vec![5, 6, 7, 8], 8),
        (vec![6, 2, 1, 2, 4, 5], 10),
    ];

    for (nums, ans) in tests {
        assert_eq!(Solution::max_alternating_sum(nums), ans);
    }
}
