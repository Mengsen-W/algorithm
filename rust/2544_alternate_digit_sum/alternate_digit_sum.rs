/*
 * @Date: 2023-07-12
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-07-12
 * @FilePath: /algorithm/rust/2544_alternate_digit_sum/alternate_digit_sum.rs
 */

struct Solution;
impl Solution {
    pub fn alternate_digit_sum(mut n: i32) -> i32 {
        let mut sign = 1;
        let mut ans = 0;
        while n > 0 {
            ans += n % 10 * sign;
            sign *= -1;
            n /= 10;
        }
        ans * -sign
    }
}

fn main() {
    let tests = vec![(521, 4), (111, 1), (886996, 0)];
    for (n, ans) in tests {
        assert_eq!(Solution::alternate_digit_sum(n), ans);
    }
}
