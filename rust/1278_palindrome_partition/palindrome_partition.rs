struct Solution;

impl Solution {
    pub fn palindrome_partition(s: String, k: i32) -> i32 {
        use std::cmp::min;
        let n = s.len();
        let mut cost = vec![vec![0; n]; n];
        for span in 2..=n {
            for i in 0..=n - span {
                let j = i + span - 1;
                cost[i][j] = cost[i + 1][j - 1]
                    + if s.as_bytes()[i] == s.as_bytes()[j] {
                        0
                    } else {
                        1
                    };
            }
        }
        let mut f = vec![vec![i32::MAX; k as usize + 1]; n + 1];
        f[0][0] = 0;
        for i in 1..=n {
            for j in 1..=std::cmp::min(k as usize, i) {
                if j == 1 {
                    f[i][j] = cost[0][i - 1];
                } else {
                    for i0 in (j - 1)..i {
                        f[i][j] = min(f[i][j], f[i0][j - 1] + cost[i0][i - 1]);
                    }
                }
            }
        }
        f[n][k as usize]
    }
}

fn main() {
    let tests = vec![("abc", 2, 1), ("aabbc", 3, 0), ("leetcode", 8, 0)];

    for (s, k, ans) in tests {
        assert_eq!(Solution::palindrome_partition(s.to_string(), k), ans);
    }
}
