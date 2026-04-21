/*
 * @Date: 2024-03-19
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-03-19
 * @FilePath: /algorithm/rust/1793_maximum_score/maximum_score.rs
 */

struct Solution;

impl Solution {
    pub fn maximum_score(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len() as i32;
        let mut left = k as i32 - 1;
        let mut right = k + 1;
        let mut ans = 0;
        let mut i = nums[k as usize];
        loop {
            while left >= 0 && left < n && nums[left as usize] >= i {
                left -= 1;
            }
            while right < n && nums[right as usize] >= i {
                right += 1;
            }
            ans = ans.max((right - left - 1) * i);
            if left == -1 && right == n {
                break;
            }
            let lval = if left == -1 { -1 } else { nums[left as usize] };
            let rval = if right == n { -1 } else { nums[right as usize] };
            i = lval.max(rval);
            if i == -1 {
                break;
            }
        }
        ans
    }
}

fn main() {
    let tests = vec![
        (vec![1, 4, 3, 7, 4, 5], 3, 15),
        (vec![5, 5, 4, 5, 4, 1, 1, 1], 0, 20),
    ];

    for (nums, k, ans) in tests {
        assert_eq!(Solution::maximum_score(nums, k), ans);
    }
}
