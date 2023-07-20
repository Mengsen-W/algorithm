/*
 * @Date: 2023-07-20
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-07-20
 * @FilePath: /algorithm/rust/918_max_subarray_sum_circular/max_subarray_sum_circular.rs
 */

struct Solution;
impl Solution {
    pub fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
        let (mut max, mut min) = (nums[0], nums[0]);
        let (mut cur_max, mut cur_min) = (0, 0);
        let mut sum = 0;
        for n in nums {
            sum += n;
            cur_max = n.max(cur_max + n);
            max = max.max(cur_max);
            cur_min = n.min(cur_min + n);
            min = min.min(cur_min);
        }
        return if max > 0 { max.max(sum - min) } else { max };
    }
}

fn main() {
    let tests = vec![
        (vec![1, -2, 3, -2], 3),
        (vec![5, -3, 5], 10),
        (vec![3, -2, 2, -3], 3),
    ];
    for (nums, ans) in tests {
        assert_eq!(Solution::max_subarray_sum_circular(nums), ans);
    }
}
