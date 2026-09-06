struct Solution;

impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let mut f = if t.len() <= s.len() {
            vec![0; t.len() + 1]
        } else {
            return 0;
        };
        f[0] = 1;

        s.chars().for_each(|sc| {
            t.chars().rev().enumerate().for_each(|(i, tc)| {
                f[t.len() - i] += if sc == tc { f[t.len() - i - 1] } else { 0 }
            });
        });

        f[t.len()]
    }
}

fn main() {
    let tests = vec![("rabbbit", "rabbit", 3), ("babgbag", "bag", 5)];

    for (s, t, expected) in tests {
        assert_eq!(Solution::num_distinct(s.into(), t.into()), expected);
    }
}
