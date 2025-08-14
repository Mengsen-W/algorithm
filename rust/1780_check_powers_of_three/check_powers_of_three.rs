struct Solution;

impl Solution {
    pub fn check_powers_of_three(n: i32) -> bool {
        let mut n = n;
        while n > 0 {
            if n % 3 == 2 {
                return false;
            }
            n /= 3;
        }
        return true;
    }
}

fn main() {
    let tests = vec![(12, true), (91, true), (21, false)];

    for (n, expected) in tests {
        assert_eq!(Solution::check_powers_of_three(n), expected);
    }
}
