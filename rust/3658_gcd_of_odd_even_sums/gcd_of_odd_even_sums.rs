struct Solution;

impl Solution {
    pub fn gcd_of_odd_even_sums(n: i32) -> i32 {
        n
    }
}

fn main() {
    let tests = vec![(4, 4), (5, 5)];

    for (n, expected) in tests {
        assert_eq!(Solution::gcd_of_odd_even_sums(n), expected);
    }
}
