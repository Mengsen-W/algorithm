/*
 * @Date: 2023-11-18
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-11-18
 * @FilePath: /algorithm/rust/2342_maximum_sum/maximum_sum.rs
 */

struct Solution;

impl Solution {
    pub fn maximum_sum(nums: Vec<i32>) -> i32 {
        let mut ans = -1;
        let mut mx = [0; 82];
        for num in nums {
            let mut s = 0;
            let mut x = num;
            while x > 0 {
                s += x % 10;
                x /= 10;
            }
            let s = s as usize;
            if mx[s] > 0 {
                ans = ans.max(mx[s] + num);
            }
            mx[s] = mx[s].max(num);
        }
        ans
    }
}

fn main() {
    let tests = vec![(vec![18, 43, 36, 13, 7], 54), (vec![10, 12, 19, 14], -1)];

    for (nums, ans) in tests {
        assert_eq!(Solution::maximum_sum(nums), ans);
    }
}
