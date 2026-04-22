struct Solution;

impl Solution {
    pub fn remove_stars(s: String) -> String {
        let mut res = String::new();
        for c in s.chars() {
            if c != '*' {
                res.push(c);
            } else {
                res.pop();
            }
        }
        res
    }
}

fn main() {
    let tests = vec![("leet**cod*e", "lecoe"), ("erase*****", "")];

    for (s, ans) in tests {
        assert_eq!(Solution::remove_stars(s.to_string()), ans.to_string());
    }
}
