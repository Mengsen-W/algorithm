/*
 * @Date: 2023-11-20
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-11-20
 * @FilePath: /algorithm/rust/53_max_sub_array/max_sub_array.rs
 */

struct Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut ans = i32::MIN;
        let mut min_pre_sum = 0;
        let mut pre_sum = 0;
        for &x in &nums {
            pre_sum += x; // 当前的前缀和
            ans = ans.max(pre_sum - min_pre_sum); // 减去前缀和的最小值
            min_pre_sum = min_pre_sum.min(pre_sum); // 维护前缀和的最小值
        }
        ans
    }
}

fn main() {
    let tests = vec![
        (vec![-2, 1, -3, 4, -1, 2, 1, -5, 4], 6),
        (vec![1], 1),
        (vec![5, 4, -1, 7, 8], 23),
    ];

    for (nums, ans) in tests {
        assert_eq!(Solution::max_sub_array(nums), ans);
    }
}
