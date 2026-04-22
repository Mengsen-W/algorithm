struct Solution;

impl Solution {
    pub fn percentage_letter(s: String, letter: char) -> i32 {
        (s.bytes().filter(|&c| c == letter as u8).count() * 100 / s.len()) as _
    }
}

fn main() {
    let tests = vec![("foobar", 'o', 33), ("jjjj", 'k', 0)];

    for (s, letter, ans) in tests {
        assert_eq!(Solution::percentage_letter(s.to_string(), letter), ans);
    }
}
