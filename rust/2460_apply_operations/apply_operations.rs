/*
 * @Date: 2023-06-05
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-06-05
 * @FilePath: /algorithm/rust/2460_apply_operations/apply_operations.rs
 */

pub fn apply_operations(mut nums: Vec<i32>) -> Vec<i32> {
    for i in 1_usize..nums.len() {
        if nums[i - 1] == nums[i] {
            nums[i - 1] *= 2;
            nums[i] = 0;
        }
    }
    for _ in 0_usize..nums.len() {
        for j in 0_usize..nums.len() - 1_usize {
            if nums[j] == 0 && nums[j + 1_usize] != 0 {
                nums.swap(j, j + 1_usize);
            }
        }
    }
    nums
}

fn main() {
    {
        let nums = vec![1, 2, 2, 1, 1, 0];
        let ans = vec![1, 4, 2, 0, 0, 0];
        assert_eq!(apply_operations(nums), ans);
    }

    {
        let nums = vec![0, 1];
        let ans = vec![1, 0];
        assert_eq!(apply_operations(nums), ans);
    }
}
