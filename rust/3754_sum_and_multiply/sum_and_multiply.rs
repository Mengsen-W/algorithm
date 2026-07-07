struct Solution;

impl Solution {
    pub fn sum_and_multiply(n: i32) -> i64 {
        let mut n = n;
        let mut x: i64 = 0;
        let mut sum: i64 = 0;
        let mut pow10: i64 = 1;
        while n > 0 {
            let d = n % 10;
            sum += d as i64;
            if d > 0 {
                x += (d as i64) * pow10;
                pow10 *= 10;
            }
            n /= 10;
        }
        x * sum
    }
}

fn main() {
    let tests = vec![(10203004, 12340), (1000, 1)];

    for (n, expected) in tests {
        assert_eq!(Solution::sum_and_multiply(n), expected);
    }
}
