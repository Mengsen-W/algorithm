struct Solution;

impl Solution {
    pub fn flower_game(n: i32, m: i32) -> i64 {
        (m as i64) * (n as i64) / 2
    }
}

fn main() {
    let tests = vec![(3, 2, 3), (1, 1, 0)];

    for (n, m, expected) in tests {
        assert_eq!(Solution::flower_game(n, m), expected);
    }
}
