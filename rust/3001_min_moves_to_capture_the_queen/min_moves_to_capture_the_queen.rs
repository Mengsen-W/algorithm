struct Solution;

impl Solution {
    pub fn min_moves_to_capture_the_queen(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32) -> i32 {
        use std::cmp::{max, min};
        // 车与皇后处在同一行，且中间没有象
        if a == e && (c != a || d <= min(b, f) || d >= max(b, f)) {
            return 1;
        }
        // 车与皇后处在同一列，且中间没有象
        if b == f && (d != b || c <= min(a, e) || c >= max(a, e)) {
            return 1;
        }
        // 象、皇后处在同一条对角线，且中间没有车
        if (c - e).abs() == (d - f).abs()
            && ((c - e) * (b - f) != (a - e) * (d - f) || a < min(c, e) || a > max(c, e))
        {
            return 1;
        }
        2
    }
}

fn main() {
    let tests = vec![(1, 1, 8, 8, 2, 3, 2), (5, 3, 3, 4, 5, 2, 1)];

    for (a, b, c, d, e, f, ans) in tests {
        assert_eq!(
            Solution::min_moves_to_capture_the_queen(a, b, c, d, e, f),
            ans
        );
    }
}
