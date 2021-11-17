/*
 * @Date: 2021-11-16 23:38:52
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-11-17 00:03:10
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
    assert_eq!(
        Solution::max_product(vec![
            "abcw".to_string(),
            "baz".to_string(),
            "foo".to_string(),
            "bar".to_string(),
            "xtfn".to_string(),
            "abcdef".to_string(),
        ]),
        16
    );

    assert_eq!(
        Solution::max_product(vec![
            "a".to_string(),
            "ab".to_string(),
            "abc".to_string(),
            "d".to_string(),
            "cd".to_string(),
            "bcd".to_string(),
            "abcd".to_string(),
        ]),
        4
    );

    assert_eq!(
        Solution::max_product(vec![
            "a".to_string(),
            "aa".to_string(),
            "aaa".to_string(),
            "aaaa".to_string(),
        ]),
        0
    );
}
