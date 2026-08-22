struct Solution;

impl Solution {
    pub fn check_divisibility(n: i32) -> bool {
        let mut digit_sum = 0;
        let mut digit_product = 1;
        let original = n;
        let mut n = n;

        while n > 0 {
            let digit = n % 10;
            n /= 10;

            digit_sum += digit;
            digit_product *= digit;
        }

        original % (digit_sum + digit_product) == 0
    }
}

fn main() {
    let tests = vec![(99, true), (23, false)];

    for (n, expected) in tests {
        assert_eq!(Solution::check_divisibility(n), expected);
    }
}
