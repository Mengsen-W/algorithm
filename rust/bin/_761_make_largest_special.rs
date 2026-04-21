struct Solution;

impl Solution {
    pub fn make_largest_special(s: String) -> String {
        let mut cnt = 0;
        let mut v: Vec<_> = s
            .split_inclusive(|c| {
                cnt += if '1' == c { 1 } else { -1 };
                cnt == 0
            })
            .map(|t| format!("1{}0", Self::make_largest_special(t[1..t.len() - 1].into())))
            .collect();

        v.sort_unstable_by(|a, b| b.cmp(a));
        v.concat()
    }
}

fn main() {
    let tests = vec![("11011000", "11100100"), ("10", "10")];

    for (s, ans) in tests {
        assert_eq!(
            Solution::make_largest_special(s.to_string()),
            ans.to_string()
        );
    }
}
