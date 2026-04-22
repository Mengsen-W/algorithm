/*
 * @Date: 2021-07-08 08:50:56
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-07-08 09:21:31
 */

fn num_subarrays_with_sum(nums: Vec<i32>, goal: i32) -> i32 {
    let n = nums.len();
    let (mut left_1, mut left_2, mut right, mut sum_1, mut sum_2, mut ret) = (0, 0, 0, 0, 0, 0);
    while right < n {
        sum_1 += nums[right as usize];
        while left_1 <= right && sum_1 > goal {
            sum_1 -= nums[left_1 as usize];
            left_1 += 1;
        }
        sum_2 += nums[right as usize];
        while left_2 <= right && sum_2 >= goal {
            sum_2 -= nums[left_2 as usize];
            left_2 += 1;
        }

        ret += left_2 - left_1;
        right += 1;
    }
    ret as i32
}

fn main() {
    {
        let nums = vec![1, 0, 1, 0, 1];
        let goal = 2;
        assert_eq!(num_subarrays_with_sum(nums, goal), 4);
    }
    {
        let nums = vec![0; 5];
        let goal = 0;
        assert_eq!(num_subarrays_with_sum(nums, goal), 15);
    }
}
