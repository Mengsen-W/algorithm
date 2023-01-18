/*
 * @Date: 2022-02-01 04:02:44
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-02-01 05:39:43
 */

pub fn longest_nice_substring(s: String) -> String {
    let s = s.chars().collect::<Vec<char>>();
    let (mut max_pos, mut max_len) = (0, 0);
    let mut check = |type_num: i32| {
        let (mut lower_cnt, mut upper_cnt) = (vec![0; 26], vec![0; 26]);
        let mut cnt = 0;
        let (mut l, mut r, mut total) = (0, 0, 0);
        while r < s.len() {
            let mut idx: usize = s[r].to_lowercase().collect::<Vec<_>>()[0] as usize - 'a' as usize;
            if s[r].is_lowercase() {
                lower_cnt[idx] += 1;
                if lower_cnt[idx] == 1 && upper_cnt[idx] > 0 {
                    cnt += 1;
                }
            } else {
                upper_cnt[idx] += 1;
                if upper_cnt[idx] == 1 && lower_cnt[idx] > 0 {
                    cnt += 1;
                }
            }
            total += if (lower_cnt[idx] + upper_cnt[idx]) == 1 {
                1
            } else {
                0
            };

            while total > type_num {
                idx = s[l].to_lowercase().collect::<Vec<_>>()[0] as usize - 'a' as usize;
                total -= if lower_cnt[idx] + upper_cnt[idx] == 1 {
                    1
                } else {
                    0
                };
                if s[l].is_lowercase() {
                    lower_cnt[idx] -= 1;
                    if lower_cnt[idx] == 0 && upper_cnt[idx] > 0 {
                        cnt -= 1;
                    }
                } else {
                    upper_cnt[idx] -= 1;
                    if upper_cnt[idx] == 0 && lower_cnt[idx] > 0 {
                        cnt -= 1;
                    }
                }
                l += 1;
            }

            if cnt == type_num && r - l + 1 > max_len {
                max_pos = l;
                max_len = r - l + 1;
            }

            r += 1;
        }
    };

    let mut mask: i32 = 0;
    for ch in &s {
        mask |= 1 << (ch.to_lowercase().collect::<Vec<_>>()[0] as usize - 'a' as usize);
    }
    let types = mask.count_ones() as i32;
    for i in 1..=types {
        check(i);
    }
    s[max_pos..max_pos + max_len].iter().collect::<String>()
}

fn main() {
    assert_eq!(
        longest_nice_substring("YazaAay".to_string()),
        "aAa".to_string()
    );

    assert_eq!(longest_nice_substring("Bb".to_string()), "Bb".to_string());

    assert_eq!(longest_nice_substring("c".to_string()), "".to_string());

    assert_eq!(
        longest_nice_substring("dDzeE".to_string()),
        "dD".to_string()
    );
}
