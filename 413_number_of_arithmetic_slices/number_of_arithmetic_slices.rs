/*
 * @Date: 2021-08-10 19:50:01
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-08-10 20:09:45
 */

struct Solution;

impl Solution {
    pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 1 {
            return 0;
        }

        let mut d = nums[0] - nums[1];
        let (mut t, mut ans) = (0, 0);

        for i in 2..n {
            if nums[i - 1] - nums[i] == d {
                t += 1;
            } else {
                d = nums[i - 1] - nums[i];
                t = 0;
            }
            ans += t;
        }
        ans
    }
}

fn main() {
    {
        let nums = vec![1, 2, 3, 4];
        assert_eq!(Solution::number_of_arithmetic_slices(nums), 3);
    }
    {
        let nums = vec![1];
        assert_eq!(Solution::number_of_arithmetic_slices(nums), 0);
    }
}
