/*
 * @Date: 2022-11-24
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-11-24
 * @FilePath: /algorithm/795_num_subarray_bounded_max/num_subarray_bounded_max.rs
 */

pub fn num_subarray_bounded_max(nums: Vec<i32>, left: i32, right: i32) -> i32 {
    let (mut res, mut last2, mut last1): (i32, i32, i32) = (0, -1, -1);
    for i in 0..nums.len() {
        if nums[i] >= left && nums[i] <= right {
            last1 = i as i32;
        } else if nums[i] > right {
            last2 = i as i32;
            last1 = -1;
        }

        if last1 != -1 {
            res += last1 - last2;
        }
    }
    res
}

fn main() {
    {
        let nums = vec![2, 1, 4, 3];
        let (left, right) = (2, 3);
        let ans = 3;
        assert_eq!(num_subarray_bounded_max(nums, left, right), ans);
    }

    {
        let nums = vec![2, 9, 2, 5, 6];
        let (left, right) = (2, 8);
        let ans = 7;
        assert_eq!(num_subarray_bounded_max(nums, left, right), ans);
    }
}
