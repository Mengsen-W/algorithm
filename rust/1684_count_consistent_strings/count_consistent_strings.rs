/*
 * @Date: 2022-11-08
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-11-08
 * @FilePath: /algorithm/1684_count_consistent_strings/count_consistent_strings.rs
 */

pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
    let mut mask = 0;
    for c in allowed.chars() {
        mask |= 1 << (c as u8 - 'a' as u8);
    }
    let mut res = 0;
    for word in words {
        let mut mask1 = 0;
        for c in word.chars() {
            mask1 |= 1 << (c as u8 - 'a' as u8);
        }
        if (mask1 | mask) == mask {
            res += 1;
        }
    }
    return res;
}

fn main() {
    {
        let allowed = String::from("ab");
        let words = vec!["ad", "bd", "aaab", "baa", "badab"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let ans = 2;
        assert_eq!(count_consistent_strings(allowed, words), ans);
    }

    {
        let allowed = String::from("abc");
        let words = vec!["a", "b", "c", "ab", "ac", "bc", "abc"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let ans = 7;
        assert_eq!(count_consistent_strings(allowed, words), ans);
    }

    {
        let allowed = String::from("cad");
        let words = vec!["cc", "acd", "b", "ba", "bac", "bad", "ac", "d"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let ans = 4;
        assert_eq!(count_consistent_strings(allowed, words), ans);
    }
}
