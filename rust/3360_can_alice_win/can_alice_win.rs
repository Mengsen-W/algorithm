struct Solution;

impl Solution {
    pub fn can_alice_win(n: i32) -> bool {
        let k = ((21.0 - ((441 - 8 * n) as f64).sqrt()) / 2.0) as i32;
        k % 2 == 1
    }
}

fn main() {
    let tests = vec![(12, true), (1, false)];

    for (test, expected) in tests {
        assert_eq!(Solution::can_alice_win(test), expected);
    }
}
