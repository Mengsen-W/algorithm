struct Solution;

impl Solution {
    pub fn minimum_pushes(word: String) -> i32 {
        let mut freq = vec![0; 26];
        for c in word.chars() {
            freq[(c as u8 - b'a') as usize] += 1;
        }

        freq.sort_by(|a, b| b.cmp(a));

        let mut ans = 0;
        for i in 0..26 {
            if freq[i] == 0 {
                break;
            }
            ans += (freq[i] * (i / 8 + 1)) as i32;
        }
        ans
    }
}

fn main() {
    let tests = vec![
        ("abcde", 5),
        ("xyzxyzxyzxyz", 12),
        ("aabbccddeeffgghhiiiiii", 24),
    ];

    for (word, expected) in tests {
        assert_eq!(Solution::minimum_pushes(word.to_string()), expected);
    }
}
