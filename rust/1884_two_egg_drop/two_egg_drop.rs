struct Solution;

impl Solution {
    pub fn two_egg_drop(n: i32) -> i32 {
        ((-1.0 + (1.0 + 8.0 * n as f64).sqrt()) / 2.0).ceil() as i32
    }
}

fn main() {
    let tests = vec![(2, 2), (100, 14)];

    for (n, ans) in tests {
        assert_eq!(Solution::two_egg_drop(n), ans);
    }
}
