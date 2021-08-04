/*
 * @Date: 2021-08-04 14:59:33
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-08-04 15:10:03
 */

struct Solution;

impl Solution {
    pub fn triangle_number(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();
        nums.sort_unstable();
        let mut ans: i32 = 0;
        for i in 0..n {
            let mut k = i;
            for j in (i + 1)..n {
                while k + 1 < n && nums[k + 1] < nums[i] + nums[j] {
                    k += 1;
                }
                ans += 0.max(k as i32 - j as i32);
            }
        }
        ans
    }
}

fn main() {
    let nums = vec![2, 2, 3, 4];
    assert_eq!(Solution::triangle_number(nums), 3);
    let nums = vec![0, 0];
    assert_eq!(Solution::triangle_number(nums), 0);
}
