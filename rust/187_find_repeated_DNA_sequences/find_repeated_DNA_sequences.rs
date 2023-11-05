/*
 * @Date: 2021-10-08 00:17:59
 * @Author: Mengsen Wang
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-11-05
 */

struct Solution;

impl Solution {
    pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        use std::collections::HashMap;
        const L: usize = 10;
        let mut bin: HashMap<char, i32> = HashMap::new();
        bin.insert('A', 0);
        bin.insert('C', 1);
        bin.insert('G', 2);
        bin.insert('T', 3);
        let mut ans: Vec<String> = Vec::new();
        let s: Vec<char> = s.chars().collect();
        let n = s.len();
        if n <= L {
            return ans;
        }
        let mut x: i32 = 0;
        for i in 0..L - 1 {
            x = (x << 2) | bin[&s[i]];
        }
        let mut cnt: HashMap<i32, i32> = HashMap::new();
        for i in 0..=n - L {
            x = ((x << 2) | bin[&s[i + L - 1]]) & ((1 << (L * 2)) - 1);
            *cnt.entry(x).or_insert(0) += 1;
            if cnt.get(&x) == Some(&2) {
                ans.push(s[i..i + L].iter().collect());
            }
        }
        ans
    }
}

fn main() {
    let tests = vec![
        (
            "AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT",
            vec!["AAAAACCCCC", "CCCCCAAAAA"],
        ),
        ("AAAAAAAAAAAAA", vec!["AAAAAAAAAA"]),
    ];

    for (s, ans) in tests {
        assert_eq!(
            Solution::find_repeated_dna_sequences(s.to_string()),
            ans.iter().map(|x| x.to_string()).collect::<Vec<String>>()
        );
    }
}
