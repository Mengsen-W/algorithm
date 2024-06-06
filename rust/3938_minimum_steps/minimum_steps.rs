struct Solution;

impl Solution {
    pub fn minimum_steps(s: String) -> i64 {
        let mut ans = 0i64;
        let mut sum = 0i64;
        for &item in s.chars().collect::<Vec<_>>().iter() {
            if item == '1' {
                sum += 1;
            } else {
                ans += sum;
            }
        }
        ans
    }
}

fn main() {
    let tests = vec![("101", 1), ("100", 2), ("0111", 0)];

    for (s, ans) in tests {
        assert_eq!(Solution::minimum_steps(s.to_string()), ans);
    }
}
