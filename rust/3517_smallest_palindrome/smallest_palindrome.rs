struct Solution;

impl Solution {
    pub fn smallest_palindrome(s: String) -> String {
        let bytes = s.as_bytes();
        let partition = bytes.len() / 2;
        let mut bucket = [0; 26];

        for i in 0..partition {
            bucket[(bytes[i] - b'a') as usize] += 1;
        }

        let mut left = String::new();
        for i in 0..26 {
            if bucket[i] > 0 {
                left.push_str(&((i as u8 + b'a') as char).to_string().repeat(bucket[i]));
            }
        }

        let mid = if bytes.len() % 2 != 0 {
            (bytes[partition] as char).to_string()
        } else {
            String::new()
        };

        let right: String = left.chars().rev().collect();

        left + &mid + &right
    }
}

fn main() {
    let tests = vec![("z", "z"), ("babab", "abbba"), ("daccad", "acddca")];

    for (s, expected) in tests {
        assert_eq!(Solution::smallest_palindrome(s.to_string()), expected);
    }
}
