struct Solution;

impl Solution {
    pub fn last_substring(s: String) -> String {
        let s = s.as_bytes();
        let n = s.len();
        let mut i = 0;
        let mut j = 1;
        while j < n {
            let mut k = 0;
            while j + k < n && s[i + k] == s[j + k] {
                k += 1;
            }
            if j + k < n && s[i + k] < s[j + k] {
                let t = i;
                i = j;
                j = std::cmp::max(j + 1, t + k + 1);
            } else {
                j = j + k + 1;
            }
        }
        String::from_utf8(s[i..].to_vec()).unwrap()
    }

    pub fn answer_string(word: String, num_friends: i32) -> String {
        if num_friends == 1 {
            return word.to_string();
        }
        let n = word.len();
        let last = Self::last_substring(word);
        let m = last.len();
        let len = std::cmp::min(m, n - num_friends as usize + 1);
        last[..len].to_string()
    }
}

fn main() {
    let tests = vec![("dbca", 2, "dbc"), ("gggg", 4, "g")];

    for (word, num_friends, expected) in tests {
        assert_eq!(
            Solution::answer_string(word.to_string(), num_friends),
            expected
        );
    }
}
