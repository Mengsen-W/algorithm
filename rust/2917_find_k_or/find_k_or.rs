/*
 * @Date: 2024-03-06
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-03-06
 * @FilePath: /algorithm/rust/2917_find_k_or/find_k_or.rs
 */

struct Solution;

impl Solution {
    pub fn find_k_or(nums: Vec<i32>, k: i32) -> i32 {
        let mut ans = 0;
        for i in 0..31 {
            let mut cnt = 0;
            for &num in nums.iter() {
                if (num >> i) & 1 == 1 {
                    cnt += 1;
                }
            }
            if cnt >= k {
                ans |= 1 << i;
            }
        }
        ans
    }
}

fn main() {
    let tests = vec![
        (vec![7, 12, 9, 8, 9, 15], 4, 9),
        (vec![2, 12, 1, 11, 4, 5], 6, 0),
        (vec![10, 8, 5, 9, 11, 6, 8], 1, 15),
    ];

    for (nums, k, ans) in tests {
        assert_eq!(Solution::find_k_or(nums, k), ans);
    }
}
