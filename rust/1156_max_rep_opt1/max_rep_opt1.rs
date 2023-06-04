/*
 * @Date: 2023-06-03
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-06-03
 * @FilePath: /algorithm/rust/1156_max_rep_opt1/max_rep_opt1.rs
 */

pub fn max_rep_opt1(text: String) -> i32 {
    use std::collections::HashMap;
    let text = text.chars().collect::<Vec<char>>();
    let count = text.iter().fold(HashMap::new(), |mut acc, &c| {
        acc.entry(c).and_modify(|e| *e += 1).or_insert(1);
        acc
    });
    let mut res = 0;
    let mut i = 0;
    let text_len = text.len();
    while i < text_len {
        let mut j = i;
        while j < text_len && text[j] == text[i] {
            j += 1;
        }
        let cur_cnt = j - i;

        if cur_cnt < count[&text[i]] && (j < text_len || i > 0) {
            res = res.max(cur_cnt + 1);
        }

        let mut k = j + 1;
        while k < text_len && text[k] == text[i] {
            k += 1;
        }
        res = res.max((k - i).min(count[&text[i]]));
        i = j;
    }
    res as i32
}

fn main() {
    {
        let text = "ababa".to_string();
        let ans = 3;
        assert_eq!(max_rep_opt1(text), ans);
    }

    {
        let text = "aaabaaa".to_string();
        let ans = 6;
        assert_eq!(max_rep_opt1(text), ans);
    }

    {
        let text = "aaabbaaa".to_string();
        let ans = 4;
        assert_eq!(max_rep_opt1(text), ans);
    }

    {
        let text = "abcdef".to_string();
        let ans = 1;
        assert_eq!(max_rep_opt1(text), ans);
    }
}
