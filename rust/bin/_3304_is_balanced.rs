struct Solution;

impl Solution {
    pub fn is_balanced(num: String) -> bool {
        let mut diff = 0;
        let mut sign = 1;
        for c in num.chars() {
            let d = c.to_digit(10).unwrap() as i32;
            diff += d * sign;
            sign = -sign;
        }
        diff == 0
    }
}

fn main() {
    let tests = vec![("1234", false), ("24123", true)];

    for (num, ans) in tests {
        assert_eq!(Solution::is_balanced(num.to_string()), ans);
    }
}
