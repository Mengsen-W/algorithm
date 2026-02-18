struct Solution;

impl Solution {
    pub fn has_alternating_bits(mut n: i32) -> bool {
        n ^= n >> 1;
        (n & n + 1) == 0
    }
}

fn main() {
    let tests = vec![(5, true), (7, false), (11, false)];

    for (n, ans) in tests {
        assert_eq!(Solution::has_alternating_bits(n), ans);
    }
}
