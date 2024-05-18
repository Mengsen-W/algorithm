/*
 * @Date: 2024-05-18
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-05-18
 * @FilePath: /algorithm/rust/2644_max_div_score/max_div_score.rs
 */

struct Solution;

impl Solution {
    pub fn max_div_score(nums: Vec<i32>, divisors: Vec<i32>) -> i32 {
        let mut cnt = -1;
        let mut ans = 0;

        for i in 0..divisors.len() {
            let mut tmp = 0;
            for j in 0..nums.len() {
                if nums[j] % divisors[i] == 0 {
                    tmp += 1;
                }
            }

            if tmp > cnt || tmp == cnt && divisors[i] < ans {
                cnt = tmp;
                ans = divisors[i];
            }
        }
        return ans;
    }
}

fn main() {
    let tests = vec![
        (vec![4, 7, 9, 3, 9], vec![5, 2, 3], 3),
        (vec![20, 14, 21, 10], vec![5, 7, 5], 5),
        (vec![12], vec![10, 16], 10),
    ];

    for (nums, divisors, ans) in tests {
        assert_eq!(Solution::max_div_score(nums, divisors), ans);
    }
}
