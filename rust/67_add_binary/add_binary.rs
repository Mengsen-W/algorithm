struct Solution;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let a_chars: Vec<char> = a.chars().collect();
        let b_chars: Vec<char> = b.chars().collect();

        let mut i = a_chars.len() as i32 - 1;
        let mut j = b_chars.len() as i32 - 1;
        let mut carry = 0;
        let mut result = Vec::new();

        while i >= 0 || j >= 0 || carry > 0 {
            let mut sum = carry;
            if i >= 0 {
                sum += a_chars[i as usize].to_digit(2).unwrap_or(0);
                i -= 1;
            }
            if j >= 0 {
                sum += b_chars[j as usize].to_digit(2).unwrap_or(0);
                j -= 1;
            }
            result.push(char::from_digit(sum % 2, 10).unwrap());
            carry = sum / 2;
        }

        result.iter().rev().collect()
    }
}

fn main() {
    let tests = vec![("11", "1", "100"), ("1010", "1011", "10101")];

    for (a, b, ans) in tests {
        assert_eq!(
            Solution::add_binary(a.to_string(), b.to_string()),
            ans.to_string()
        );
    }
}
