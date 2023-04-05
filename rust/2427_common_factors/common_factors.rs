/*
 * @Date: 2023-04-05
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-04-05
 * @FilePath: /algorithm/rust/2427_common_factors/common_factors.rs
 */

pub fn common_factors(a: i32, b: i32) -> i32 {
    (1..=a.min(b)).fold(0, |prev, v| {
        prev + if a % v == 0 && b % v == 0 { 1 } else { 0 }
    })
}

fn main() {
    {
        let (a, b, ans) = (12, 6, 4);
        assert_eq!(common_factors(a, b), ans);
    }

    {
        let (a, b, ans) = (25, 30, 2);
        assert_eq!(common_factors(a, b), ans);
    }
}
