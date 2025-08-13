struct Solution;

impl Solution {
    pub fn is_power_of_three(mut n: i32) -> bool {
        while n > 0 && n % 3 == 0 {
            n /= 3
        }
        n == 1
    }
}

fn main() {
    let tests = vec![(27, true), (0, false), (9, true), (45, false)];
    for (n, expected) in tests {
        assert_eq!(Solution::is_power_of_three(n), expected);
    }
}
