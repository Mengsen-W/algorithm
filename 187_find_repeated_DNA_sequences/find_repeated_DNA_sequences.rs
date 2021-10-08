/*
 * @Date: 2021-10-08 00:17:59
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-10-08 11:28:52
 */

struct Solution;

impl Solution {
    // pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
    //     use std::collections::HashMap;
    //     let s: Vec<char> = s.chars().collect();
    //     let mut tab = HashMap::with_capacity(s.len() << 1);
    //     for w in s.windows(10) {
    //         *tab.entry(w).or_insert(0) += 1;
    //     }
    //     tab.into_iter()
    //         .filter(|&(_, cnt)| cnt > 1)
    //         .map(|(s, _)| s.iter().collect::<String>())
    //         .collect()
    // }
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
    {
        let s: String = "AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT".to_string();
        let ans: Vec<String> = ["AAAAACCCCC", "CCCCCAAAAA"]
            .iter()
            .map(|x| x.to_string())
            .collect();
        assert_eq!(Solution::find_repeated_dna_sequences(s), ans);
    }
    {
        let s: String = "AAAAAAAAAAAAA".to_string();
        let ans: Vec<String> = vec!["AAAAAAAAAA".to_string()];
        assert_eq!(Solution::find_repeated_dna_sequences(s), ans);
    }
}
