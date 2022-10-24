/*
 * @Date: 2022-10-24
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-10-24
 * @FilePath: /algorithm/915_partition_disjoint/partition_disjoint.rs
 */

pub fn partition_disjoint(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let (mut left_max, mut left_pos, mut cur_max) = (nums[0], 0, nums[0]);
    for i in 1..(n - 1) {
        cur_max = cur_max.max(nums[i]);
        if nums[i] < left_max {
            left_max = cur_max;
            left_pos = i as i32;
        }
    }
    left_pos + 1
}

fn main() {
    {
        let nums = vec![5, 0, 3, 8, 6];
        let ans = 3;
        assert_eq!(partition_disjoint(nums), ans);
    }

    {
        let nums = vec![1, 1, 1, 0, 6, 12];
        let ans = 4;
        assert_eq!(partition_disjoint(nums), ans);
    }
}
