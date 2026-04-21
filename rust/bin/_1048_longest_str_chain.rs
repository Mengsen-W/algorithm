/*
 * @Date: 2023-04-27
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-04-27
 * @FilePath: /algorithm/rust/1048_longest_str_chain/longest_str_chain.rs
 */

pub fn longest_str_chain(mut words: Vec<String>) -> i32 {
    use std::collections::HashMap;
    let mut ans = 0;
    let mut dp: HashMap<String, i32> = HashMap::new();
    words.sort_by_key(|w| w.len());
    for w in words.iter() {
        let maxdp = (0..w.len())
            .map(|i| *dp.entry(w[..i].to_string() + &w[i + 1..]).or_default())
            .max()
            .unwrap();
        dp.insert(w.to_string(), maxdp + 1);
        ans = ans.max(dp[w]);
    }
    ans
}

fn main() {
    {
        let words = vec!["a", "b", "ba", "bca", "bda", "bdca"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let ans = 4;
        assert_eq!(longest_str_chain(words), ans);
    }

    {
        let words = vec!["xbc", "pcxbcf", "xb", "cxbc", "pcxbc"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let ans = 5;
        assert_eq!(longest_str_chain(words), ans);
    }

    {
        let words = vec!["abcd", "dbqca"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let ans = 1;
        assert_eq!(longest_str_chain(words), ans);
    }
}
