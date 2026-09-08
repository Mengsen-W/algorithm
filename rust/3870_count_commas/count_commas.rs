struct Solution;

impl Solution {
    pub fn count_commas(n: i32) -> i32 {
        0.max(n - 999)
    }
}

fn main() {
    let tests = vec![(1002, 3), (998, 0)];

    for (n, ans) in tests {
        assert_eq!(Solution::count_commas(n), ans);
    }
}
