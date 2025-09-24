struct Solution;

impl Solution {
    fn is_vowel(c: usize) -> bool {
        matches!(c as u8 + b'a', b'a' | b'e' | b'i' | b'o' | b'u')
    }

    pub fn max_freq_sum(s: String) -> i32 {
        let mut freq = [0; 26];
        for ch in s.chars() {
            freq[(ch as u8 - b'a') as usize] += 1;
        }

        let (mut vowel, mut consonant) = (0, 0);
        for (i, &f) in freq.iter().enumerate() {
            if Self::is_vowel(i) {
                vowel = vowel.max(f);
            } else {
                consonant = consonant.max(f);
            }
        }
        vowel + consonant
    }
}

fn main() {
    let tests = vec![("successes", 6), ("aeiaeia", 3)];

    for (s, expected) in tests {
        assert_eq!(Solution::max_freq_sum(s.to_string()), expected);
    }
}
