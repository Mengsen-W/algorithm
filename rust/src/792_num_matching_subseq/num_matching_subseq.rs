/*
 * @Date: 2022-11-17
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-11-17
 * @FilePath: /algorithm/792_num_matching_subseq/num_matching_subseq.rs
 */

pub fn num_matching_subseq(s: String, words: Vec<String>) -> i32 {
    let words = words
        .into_iter()
        .map(|v| v.into_bytes())
        .collect::<Vec<_>>();
    let mut heads = vec![vec![]; 26];
    for word in &words {
        heads[(word[0] - b'a') as usize].push(&word[1..]);
    }
    let mut res = 0;
    for ch in s.bytes() {
        let tails = std::mem::take(&mut heads[(ch - b'a') as usize]);
        for tail in tails {
            if tail.is_empty() {
                res += 1;
            } else {
                heads[(tail[0] - b'a') as usize].push(&tail[1..]);
            }
        }
    }
    return res;
}

fn main() {
    {
        let s = String::from("abcde");
        let words = vec!["a", "bb", "acd", "ace"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let ans = 3;
        assert_eq!(num_matching_subseq(s, words), ans);
    }

    {
        let s = String::from("dsahjpjauf");
        let words = vec!["ahjpjau", "ja", "ahbwzgqnuk", "tnmlanowax"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let ans = 2;
        assert_eq!(num_matching_subseq(s, words), ans);
    }
}
