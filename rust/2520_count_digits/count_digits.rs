/*
 * @Date: 2023-10-26
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-10-26
 * @FilePath: /algorithm/rust/2520_count_digits/count_digits.rs
 */

struct Solution;

impl Solution {
    pub fn count_digits(num: i32) -> i32 {
        let mut ans = 0;
        let mut x = num;
        while x != 0 {
            if num % (x % 10) == 0 {
                ans += 1;
            }
            x /= 10;
        }
        ans
    }
}

fn main() {
    let tests = vec![(7, 1), (121, 2), (1248, 4)];

    for (num, ans) in tests {
        assert_eq!(Solution::count_digits(num), ans);
    }
}
