struct Solution;

impl Solution {
    pub fn max_number_of_balloons(text: String) -> i32 {
        let mut cnt = [0; 5];
        text.chars().for_each(|c| match c {
            'b' => cnt[0] += 1,
            'a' => cnt[1] += 1,
            'l' => cnt[2] += 1,
            'o' => cnt[3] += 1,
            'n' => cnt[4] += 1,
            _ => (),
        });
        cnt[2] >>= 1;
        cnt[3] >>= 1;
        *cnt.iter().min().unwrap_or(&0)
    }
}

fn main() {
    let tests = vec![("nlaebolko", 1), ("loonbalxballpoon", 2), ("leetcode", 0)];

    for (text, expected) in tests {
        assert_eq!(Solution::max_number_of_balloons(text.to_string()), expected);
    }
}
