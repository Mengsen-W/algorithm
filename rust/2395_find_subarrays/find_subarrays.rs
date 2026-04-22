/*
 * @Date: 2023-03-26
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-03-26
 * @FilePath: /algorithm/rust/2395_find_subarrays/find_subarrays.rs
 */

pub fn find_subarrays(nums: Vec<i32>) -> bool {
    use std::collections::HashSet;
    let mut cnt = HashSet::new();
    for i in 1..nums.len() {
        if !cnt.insert(nums[i] + nums[i - 1]) {
            return true;
        }
    }
    false
}

fn main() {
    {
        let nums = vec![4, 2, 4];
        let ans = true;
        assert_eq!(find_subarrays(nums), ans);
    }

    {
        let nums = vec![1, 2, 3, 4, 5];
        let ans = false;
        assert_eq!(find_subarrays(nums), ans);
    }

    {
        let nums = vec![0, 0, 0];
        let ans = true;
        assert_eq!(find_subarrays(nums), ans);
    }
}
