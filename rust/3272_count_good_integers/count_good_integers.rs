struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn count_good_integers(n: i32, k: i32) -> i64 {
        let mut dict: HashSet<String> = HashSet::new();
        let base = 10i32.pow(((n - 1) / 2) as u32);
        let skip = (n & 1) as usize;
        /* 枚举 n 个数位的回文数 */
        for i in base..base * 10 {
            let s = i.to_string();
            let rev: String = s.chars().rev().skip(skip).collect();
            let combined = format!("{}{}", s, rev);
            let palindromic_integer: i64 = combined.parse().unwrap();
            /* 如果当前回文数是 k 回文数 */
            if palindromic_integer % (k as i64) == 0 {
                let mut sorted_chars: Vec<char> = combined.chars().collect();
                sorted_chars.sort();
                dict.insert(sorted_chars.into_iter().collect());
            }
        }

        let mut factorial = vec![1i64; (n + 1) as usize];
        for i in 1..=n as usize {
            factorial[i] = factorial[i - 1] * (i as i64);
        }

        let mut ans = 0i64;
        for s in dict {
            let mut cnt = vec![0; 10];
            for c in s.chars() {
                cnt[c.to_digit(10).unwrap() as usize] += 1;
            }
            /* 计算排列组合 */
            let mut tot = (n as i64 - cnt[0] as i64) * factorial[(n - 1) as usize];
            for &x in cnt.iter() {
                tot /= factorial[x];
            }
            ans += tot;
        }

        ans
    }
}

fn main() {
    let tests = vec![(3, 5, 27), (1, 4, 2), (5, 6, 2468)];

    for (n, k, expected) in tests {
        assert_eq!(Solution::count_good_integers(n, k), expected);
    }
}
