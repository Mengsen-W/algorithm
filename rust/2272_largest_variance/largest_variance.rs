struct Solution;

impl Solution {
    pub fn largest_variance(s: String) -> i32 {
        use std::cmp::max;
        use std::collections::HashMap;
        let mut pos: HashMap<char, Vec<usize>> = HashMap::new();
        for (i, ch) in s.char_indices() {
            pos.entry(ch).or_insert(Vec::new()).push(i);
        }
        let mut ans = 0;
        for (&c0, pos0) in &pos {
            for (&c1, pos1) in &pos {
                if c0 != c1 {
                    let mut i = 0;
                    let mut j = 0;
                    let mut f = 0;
                    let mut g = i32::MIN;
                    while i < pos0.len() || j < pos1.len() {
                        if j == pos1.len() || (i < pos0.len() && pos0[i] < pos1[j]) {
                            f = max(f, 0) + 1;
                            g = g + 1;
                            i += 1;
                        } else {
                            g = max(f, max(g, 0)) - 1;
                            f = max(f, 0) - 1;
                            j += 1;
                        }
                        ans = max(ans, g);
                    }
                }
            }
        }
        ans
    }
}

fn main() {
    let tests = vec![("aababbb", 3), ("abcde", 0)];

    for (s, ans) in tests {
        assert_eq!(Solution::largest_variance(s.to_string()), ans);
    }
}
