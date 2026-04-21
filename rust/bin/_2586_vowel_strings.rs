/*
 * @Date: 2023-11-07
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-11-07
 * @FilePath: /algorithm/rust/2586_vowel_strings/vowel_strings.rs
 */

struct Solution;
impl Solution {
    pub fn vowel_strings(words: Vec<String>, left: i32, right: i32) -> i32 {
        let check =
            |c: u8| -> bool { c == b'a' || c == b'e' || c == b'i' || c == b'o' || c == b'u' };

        let mut ans = 0;
        for i in left..=right {
            let w = words[i as usize].as_bytes();
            if check(w[0]) && check(w[w.len() - 1]) {
                ans += 1;
            }
        }

        ans
    }
}

fn main() {
    let tests = vec![
        (vec!["are", "amy", "u"], 0, 2, 2),
        (vec!["hey", "aeo", "mu", "ooo", "artro"], 1, 4, 3),
    ];

    for (words, left, right, ans) in tests {
        assert_eq!(
            Solution::vowel_strings(words.iter().map(|x| x.to_string()).collect(), left, right),
            ans
        );
    }
}
