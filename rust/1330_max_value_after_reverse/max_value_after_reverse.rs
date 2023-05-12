/*
 * @Date: 2023-05-12
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-05-12
 * @FilePath: /algorithm/rust/1330_max_value_after_reverse/max_value_after_reverse.rs
 */

pub fn max_value_after_reverse(nums: Vec<i32>) -> i32 {
    let mut value = 0;
    let n = nums.len();
    for i in 0..(n - 1) {
        value += (nums[i] - nums[i + 1]).abs();
    }
    let mut mx1 = 0;
    for i in 1..(n - 1) {
        mx1 = mx1.max((nums[0] - nums[i + 1]).abs() - (nums[i] - nums[i + 1]).abs());
        mx1 = mx1.max((nums[n - 1] - nums[i - 1]).abs() - (nums[i] - nums[i - 1]).abs());
    }
    let mut mx2 = i32::MIN;
    let mut mn2 = i32::MAX;

    for i in 0..(n - 1) {
        let x = nums[i];
        let y = nums[i + 1];
        mx2 = mx2.max(x.min(y));
        mn2 = mn2.min(x.max(y));
    }
    return value + mx1.max(2 * (mx2 - mn2));
}

fn main() {
    {
        let nums = vec![2, 3, 1, 5, 4];
        let ans = 10;
        assert_eq!(max_value_after_reverse(nums), ans);
    }

    {}
}
