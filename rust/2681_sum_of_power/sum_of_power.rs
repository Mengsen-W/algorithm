/*
 * @Date: 2023-08-01
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-08-01
 * @FilePath: /algorithm/rust/2681_sum_of_power/sum_of_power.rs
 */

struct Solution;
impl Solution {
    pub fn sum_of_power(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        const N: usize = 1e9 as usize + 7;
        let mut ans = 0;
        let mut buffer: usize = 0;
        for i in 0..nums.len() {
            let min_value: usize = nums[i] as usize + buffer;
            ans = (ans + ((nums[i] as usize * nums[i] as usize) % N) * min_value) % N;
            buffer = (buffer * 2) % N;
            buffer += nums[i] as usize;
        }
        ans as i32
    }
}

fn main() {
    let tests = vec![(vec![2, 1, 4], 141), (vec![1, 1, 1], 7)];
    for (nums, ans) in tests {
        assert_eq!(Solution::sum_of_power(nums), ans);
    }
}
