struct Solution;

impl Solution {
    pub fn get_smallest_string(s: String, k: i32) -> String {
        let mut k = k;
        let mut chars: Vec<char> = s.chars().collect();
        for i in 0..chars.len() {
            let dis = std::cmp::min(
                (chars[i] as u8 - 'a' as u8) as i32,
                ('z' as u8 - chars[i] as u8 + 1) as i32,
            );
            if dis <= k {
                chars[i] = 'a';
                k -= dis;
            } else {
                chars[i] = (chars[i] as u8 - k as u8) as char;
                break;
            }
        }
        chars.into_iter().collect()
    }
}

fn main() {
    let tests = vec![
        ("zbbz", 3, "aaaz"),
        ("xaxcd", 4, "aawcd"),
        ("lol", 0, "lol"),
    ];

    for (s, k, ans) in tests {
        assert_eq!(Solution::get_smallest_string(s.to_string(), k), ans);
    }
}
