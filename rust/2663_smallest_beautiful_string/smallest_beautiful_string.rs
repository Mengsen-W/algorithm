struct Solution;

impl Solution {
    fn smallest_beautiful_string(s: String, k: i32) -> String {
        let t = s.as_bytes();
        for i in (0..s.len()).rev() {
            let mut blocked_characters = Vec::new();
            for j in 1..=2 {
                if i >= j {
                    blocked_characters.push(t[i - j]);
                }
            }
            for j in 1..=3 {
                if (t[i] - b'a' + j + 1) as i32 <= k
                    && !blocked_characters.iter().any(|&x| x == t[i] + j)
                {
                    return Self::generate(&s, i, j as u8);
                }
            }
        }
        "".to_string()
    }

    fn generate(s: &String, idx: usize, offset: u8) -> String {
        let mut res: Vec<char> = s.chars().collect();
        res[idx] = ((res[idx] as u8) + offset) as char;
        for i in (idx + 1)..s.len() {
            let mut blocked_characters = Vec::new();
            for j in 1..=2 {
                if i >= j {
                    blocked_characters.push(res[i - j] as u8);
                }
            }
            for j in 0..=2 {
                if !blocked_characters.iter().any(|&x| x == (b'a' + j)) {
                    res[i] = (b'a' + j) as char;
                    break;
                }
            }
        }
        res.into_iter().collect()
    }
}

fn main() {
    let tests = vec![("abcz", 26, "abda"), ("dc", 4, "")];

    for (s, k, ans) in tests {
        assert_eq!(Solution::smallest_beautiful_string(s.to_string(), k), ans);
    }
}
