struct Solution;

impl Solution {
    pub fn remove_trailing_zeros(num: String) -> String {
        let mut num = num;
        while num.ends_with('0') {
            num.pop();
        }
        num
    }
}

fn main() {
    let tests = vec![("51230100", "512301"), ("123", "123")];

    for (num, ans) in tests {
        assert_eq!(
            Solution::remove_trailing_zeros(num.to_string()),
            ans.to_string()
        );
    }
}
