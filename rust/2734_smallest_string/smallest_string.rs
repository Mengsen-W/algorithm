struct Solution;

impl Solution {
    pub fn smallest_string(s: String) -> String {
        let target = 'a';
        let index_of_first_non_a = s.chars().position(|c| c != target).unwrap_or(s.len());
        if index_of_first_non_a == s.len() {
            return s[0..s.len() - 1].to_string() + "z";
        }
        let index_of_first_a_after_first_non_a =
            Self::find_first_a_after_first_non_a(&s, index_of_first_non_a);
        let mut res = String::new();
        for (i, c) in s.chars().enumerate() {
            if index_of_first_non_a <= i && i < index_of_first_a_after_first_non_a {
                res.push((c as u8 - 1) as char);
            } else {
                res.push(c);
            }
        }
        res
    }

    pub fn find_first_a_after_first_non_a(s: &String, index_of_first_non_a: usize) -> usize {
        let chars = s.chars().skip(index_of_first_non_a);
        for (index, c) in chars.enumerate() {
            if c == 'a' {
                return index + index_of_first_non_a;
            }
        }
        s.len()
    }
}

fn main() {
    let tests = vec![
        ("cbabc", "baabc"),
        ("acbbc", "abaab"),
        ("leetcode", "kddsbncd"),
    ];
    for (s, ans) in tests {
        assert_eq!(Solution::smallest_string(s.to_string()), ans);
    }
}
