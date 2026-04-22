/*
 * @Date: 2021-08-11 14:38:53
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-08-11 15:06:31
 */

struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let n = nums.len();
        let mut f: Vec<HashMap<i64, i32>> = vec![HashMap::new(); n];
        for i in 0..n {
            for j in 0..i {
                let d = nums[i] as i64 - nums[j] as i64;
                let mut cnt = 0;
                if let Some(&c) = f[j].get(&d) {
                    cnt = c;
                }
                ans += cnt;
                *f[i].entry(d).or_insert(0) += cnt + 1;
            }
        }
        ans
    }
}

fn main() {
    {
        let nums = vec![2, 4, 6, 8, 10];
        assert_eq!(Solution::number_of_arithmetic_slices(nums), 7);
    }
    {
        let nums = vec![7, 7, 7, 7, 7];
        assert_eq!(Solution::number_of_arithmetic_slices(nums), 16);
    }

    {
        let nums = vec![0, 2000000000, -294967296];
        assert_eq!(Solution::number_of_arithmetic_slices(nums), 0);
    }
}
