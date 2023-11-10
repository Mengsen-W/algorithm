/*
 * @Date: 2021-11-16 23:38:52
 * @Author: Mengsen Wang
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-11-10
 */

struct Solution;

impl Solution {
    pub fn max_product(words: Vec<String>) -> i32 {
        use std::collections::HashMap;
        let mut mp = HashMap::new();
        for word in words {
            let mut mask = 0;
            for x in word.as_bytes() {
                mask |= 1 << (x - b'a');
            }
            let n = word.len() as i32;
            if mp.contains_key(&mask) {
                if n > *mp.get(&mask).unwrap() {
                    mp.insert(mask, n);
                }
            } else {
                mp.insert(mask, n);
            }
        }

        let mut ans = 0;
        for (k1, v1) in &mp {
            for (k2, v2) in &mp {
                if k1 & k2 == 0 {
                    ans = ans.max(v1 * v2);
                }
            }
        }
        ans
    }
}

fn main() {
    let tests = vec![
        (vec!["abcw", "baz", "foo", "bar", "xtfn", "abcdef"], 16),
        (vec!["a", "ab", "abc", "d", "cd", "bcd", "abcd"], 4),
        (vec!["a", "aa", "aaa", "aaaa"], 0),
    ];

    for (words, ans) in tests {
        assert_eq!(
            Solution::max_product(words.iter().map(|x| x.to_string()).collect()),
            ans
        );
    }
}
