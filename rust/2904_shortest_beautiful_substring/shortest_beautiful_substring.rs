struct Solution;

impl Solution {
    pub fn shortest_beautiful_substring(s: String, k: i32) -> String {
        let bytes = s.as_bytes();
        if bytes.iter().filter(|&&b| b == b'1').count() < k as usize {
            return String::new();
        }
        let mut ans = s.clone();
        let (mut cnt, mut left) = (0, 0);
        for right in 0..bytes.len() {
            cnt += (bytes[right] - b'0') as i32;
            while cnt > k || bytes[left] == b'0' {
                cnt -= (bytes[left] - b'0') as i32;
                left += 1;
            }
            if cnt == k {
                let t = &s[left..=right];
                if t.len() < ans.len() || t.len() == ans.len() && t < ans.as_str() {
                    ans = t.to_string();
                }
            }
        }
        ans
    }
}

fn main() {
    let tests = vec![("100011001", 3, "11001"), ("1011", 2, "11"), ("000", 1, "")];

    for (s, k, ans) in tests {
        assert_eq!(
            Solution::shortest_beautiful_substring(s.to_string(), k),
            ans
        );
    }
}
