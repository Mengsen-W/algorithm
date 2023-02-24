/*
 * @Date: 2023-02-24
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-02-24
 * @FilePath: /algorithm/rust/2357_minimum_operations/minimum_operations.rs
 */

pub fn minimum_operations(nums: Vec<i32>) -> i32 {
    (nums.into_iter().fold(0, |s, x| s | 1u128 << x) >> 1).count_ones() as i32
}

fn main() {
    {
        let nums = vec![1, 5, 0, 3, 5];
        let ans = 3;
        assert_eq!(minimum_operations(nums), ans);
    }

    {
        let nums = vec![0];
        let ans = 0;
        assert_eq!(minimum_operations(nums), ans);
    }
}
