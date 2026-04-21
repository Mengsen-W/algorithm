struct Solution;

impl Solution {
    pub fn minimum_cost(s: String) -> i64 {
        s.chars()
            .collect::<Vec<_>>()
            .windows(2)
            .enumerate()
            .filter(|&(_, w)| w[0] != w[1])
            .map(|(i, _)| (i + 1).min(s.len() - (i + 1)) as i64)
            .sum()
    }
}

fn main() {
    let tests = vec![("0011", 2), ("010101", 9)];

    for (s, ans) in tests {
        assert_eq!(Solution::minimum_cost(s.to_string()), ans);
    }
}
