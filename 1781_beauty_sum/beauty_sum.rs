/*
 * @Date: 2022-12-12
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-12-12
 * @FilePath: /algorithm/1781_beauty_sum/beauty_sum.rs
 */

pub fn beauty_sum(s: String) -> i32 {
    let s: Vec<usize> = s.chars().map(|c| c as usize).collect();
    let mut res = 0;
    const A: usize = 'a' as usize;
    for i in 0..s.len() {
        let mut cnt = vec![0; 26];
        let mut max_freq = 0;

        for j in i..s.len() {
            cnt[s[j] - A] += 1;
            max_freq = max_freq.max(cnt[s[j] - A]);
            let mut min_freq = s.len();

            for k in 0..26 {
                if cnt[k] > 0 {
                    min_freq = min_freq.min(cnt[k]);
                }
            }
            res += max_freq - min_freq;
        }
    }
    res as i32
}

fn main() {
    {
        let s = String::from("aabcb");
        let ans = 5;
        assert_eq!(beauty_sum(s), ans);
    }

    {
        let s = String::from("aabcbaa");
        let ans = 17;
        assert_eq!(beauty_sum(s), ans);
    }
}
