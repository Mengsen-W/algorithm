/*
 * @Date: 2024-05-26
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-05-26
 * @FilePath: /algorithm/rust/2903_find_indices/find_indices.rs
 */

struct Solution;

impl Solution {
    pub fn find_indices(nums: Vec<i32>, index_difference: i32, value_difference: i32) -> Vec<i32> {
        let mut min_index = 0;
        let mut max_index = 0;
        for j in (index_difference as usize)..nums.len() {
            let i = j - index_difference as usize;
            if nums[i] < nums[min_index] {
                min_index = i;
            }
            if nums[j] - nums[min_index] >= value_difference {
                return vec![min_index as i32, j as i32];
            }
            if nums[i] > nums[max_index] {
                max_index = i;
            }
            if nums[max_index] - nums[j] >= value_difference {
                return vec![max_index as i32, j as i32];
            }
        }
        vec![-1, -1]
    }
}

fn main() {
    let tests = vec![
        (vec![5, 1, 4, 1], 2, 4, vec![0, 3]),
        (vec![2, 1], 0, 0, vec![0, 0]),
        (vec![1, 2, 3], 2, 4, vec![-1, -1]),
    ];

    for (nums, index_difference, value_difference, ans) in tests {
        assert_eq!(
            Solution::find_indices(nums, index_difference, value_difference),
            ans
        );
    }
}
