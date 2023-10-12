/*
 * @Date: 2023-10-12
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-10-12
 * @FilePath: /algorithm/rust/2562_find_the_array_conc_val/find_the_array_conc_val.rs
 */

struct Solution;
impl Solution {
    pub fn find_the_array_conc_val(nums: Vec<i32>) -> i64 {
        let mut ans = 0i64;
        let mut i = 0;
        let mut j = nums.len() - 1;
        while i < j {
            let mut x = nums[i];
            let mut y = nums[j];
            while y != 0 {
                x *= 10;
                y /= 10;
            }
            ans += (x + nums[j]) as i64;
            i += 1;
            j -= 1;
        }
        if i == j {
            ans += nums[i] as i64;
        }
        ans
    }
}

fn main() {
    let tests = vec![(vec![7, 52, 2, 4], 596), (vec![5, 14, 13, 8, 12], 673)];

    for (nums, ans) in tests {
        assert_eq!(Solution::find_the_array_conc_val(nums), ans);
    }
}
