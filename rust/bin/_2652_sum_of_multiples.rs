/*
 * @Date: 2023-10-17
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-10-17
 * @FilePath: /algorithm/rust/2652_sum_of_multiples/sum_of_multiples.rs
 */

struct Solution;

impl Solution {
    pub fn sum_of_multiples(n: i32) -> i32 {
        fn f(n: i32, m: i32) -> i32 {
            (m + n / m * m) * (n / m) / 2
        }
        f(n, 3) + f(n, 5) + f(n, 7) - f(n, 3 * 5) - f(n, 3 * 7) - f(n, 5 * 7) + f(n, 3 * 5 * 7)
    }
}

fn main() {
    let tests = vec![(7, 21), (10, 40), (9, 30)];

    for (n, ans) in tests {
        assert_eq!(Solution::sum_of_multiples(n), ans);
    }
}
