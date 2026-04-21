/*
 * @Date: 2021-08-03 15:41:49
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-08-03 16:51:44
 */

struct Solution;

impl Solution {
    pub fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
        Self::find_unsorted_subarray_sort(nums)
        Self::find_unsorted_subarray_traverse(nums)
    }
    fn find_unsorted_subarray_sort(nums: Vec<i32>) -> i32 {
        if Self::is_sorted(&nums) {
            return 0;
        }
        let mut nums_sorted = nums.clone();
        nums_sorted.sort_unstable();
        let (mut left, mut right) = (0, nums.len() - 1);
        while nums_sorted[left] == nums[left] {
            left += 1;
        }
        while nums[right] == nums_sorted[right] {
            right -= 1;
        }
        return (right - left + 1) as i32;
    }

    fn find_unsorted_subarray_traverse(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let (mut maxn, mut minn) = (i32::MIN, i32::MAX);
        let (mut left, mut right): (i32, i32) = (-1, -1);
        for i in 0..n {
            match maxn > nums[i] {
                true => right = i as i32,
                false => maxn = nums[i],
            }
            match minn < nums[n - i - 1] {
                true => left = (n - i - 1) as i32,
                false => minn = nums[n - i - 1],
            }
        }
        match right {
            -1 => 0,
            _ => right - left + 1,
        }
    }

    fn is_sorted(nums: &Vec<i32>) -> bool {
        nums.windows(2).all(|w| w[0] <= w[1])
    }
}

fn main() {
    {
        let nums = vec![2, 6, 4, 8, 10, 9, 15];
        let ans = 5;
        assert_eq!(Solution::find_unsorted_subarray_sort(nums.clone()), ans);
        assert_eq!(Solution::find_unsorted_subarray_traverse(nums.clone()), ans);
    }
    {
        let nums = vec![1, 2, 3, 4];
        let ans = 0;
        assert_eq!(Solution::find_unsorted_subarray_sort(nums.clone()), ans);
        assert_eq!(Solution::find_unsorted_subarray_traverse(nums.clone()), ans);
    }
    {
        let nums = vec![1];
        let ans = 0;
        assert_eq!(Solution::find_unsorted_subarray_sort(nums.clone()), ans);
        assert_eq!(Solution::find_unsorted_subarray_traverse(nums.clone()), ans);
    }
}
