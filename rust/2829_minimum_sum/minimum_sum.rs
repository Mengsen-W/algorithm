struct Solution;

impl Solution {
    pub fn minimum_sum(n: i32, k: i32) -> i32 {
        fn arithmetic_series_sum(a1: i32, d: i32, n: i32) -> i32 {
            (a1 + a1 + (n - 1) * d) * n / 2
        }
        if n <= k / 2 {
            arithmetic_series_sum(1, 1, n)
        } else {
            arithmetic_series_sum(1, 1, k / 2) + arithmetic_series_sum(k, 1, n - k / 2)
        }
    }
}

fn main() {
    let tests = vec![(5, 4, 18), (2, 6, 3)];

    for (n, k, ans) in tests {
        assert_eq!(Solution::minimum_sum(n, k), ans);
    }
}
