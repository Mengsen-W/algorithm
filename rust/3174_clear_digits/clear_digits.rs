struct Solution;

impl Solution {
    pub fn clear_digits(s: String) -> String {
        let mut res: Vec<char> = Vec::new();
        for c in s.chars() {
            if c.is_digit(10) {
                res.pop();
            } else {
                res.push(c);
            }
        }
        res.into_iter().collect()
    }
}

fn main() {
    let tests = vec![("abc", "abc"), ("cb34", "")];

    for (s, ans) in tests {
        assert_eq!(Solution::clear_digits(s.to_string()), ans.to_string());
    }
}
