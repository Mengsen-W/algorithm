struct Solution;

impl Solution {
    pub fn distinct_subseq_ii(s: String) -> i32 {
        let m = 1000_000_007;
        let mut g = vec![0; 26];
        let mut x = 0;
        let b = 'a' as u8;
        for c in s.chars() {
            let a = c as u8 - b;
            let r = g[a as usize];
            g[a as usize] = (x % m + 1) % m;
            x = (x % m + 1 + x % m - r % m) % m;
            if x < 0 {
                x += m;
            }
        }
        x
    }
}

fn main() {
    let tests = vec![("abc", 7), ("aba", 6), ("aaa", 3)];

    for (s, ans) in tests {
        assert_eq!(Solution::distinct_subseq_ii(s.to_string()), ans);
    }
}
