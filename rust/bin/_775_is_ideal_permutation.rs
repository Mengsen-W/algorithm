/*
 * @Date: 2022-11-16
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-11-16
 * @FilePath: /algorithm/775_is_ideal_permutation/is_ideal_permutation.rs
 */

pub fn is_ideal_permutation(nums: Vec<i32>) -> bool {
    for i in 0..nums.len() {
        if (nums[i] - i as i32).abs() > 1 {
            return false;
        }
    }
    return true;
}

fn main() {
    {
        let nums = vec![1, 0, 2];
        assert!(is_ideal_permutation(nums));
    }

    {
        let nums = vec![1, 2, 0];
        assert!(!is_ideal_permutation(nums));
    }
}
