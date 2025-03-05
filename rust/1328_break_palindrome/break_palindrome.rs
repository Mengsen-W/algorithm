struct Solution;

impl Solution {
    pub fn break_palindrome(palindrome: String) -> String {
        let n = palindrome.len();
        if n == 1 {
            return "".to_string();
        }
        let mut data: Vec<char> = palindrome.chars().collect();
        for i in 0..(n / 2) {
            if data[i] != 'a' {
                data[i] = 'a';
                return data.iter().collect();
            }
        }
        data[n - 1] = 'b';
        data.iter().collect()
    }
}

fn main() {
    let tests = vec![("abccba", "aaccba"), ("a", "")];

    for (palindrome, ans) in tests {
        assert_eq!(
            Solution::break_palindrome(palindrome.to_string()),
            ans.to_string()
        );
    }
}
