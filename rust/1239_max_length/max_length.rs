/*
 * @Date: 2021-06-19 09:44:11
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-06-19 10:28:40
 */

fn max_length(arr: Vec<String>) -> i32 {
    let mut ans: i32 = 0;
    let mut masks: Vec<i32> = vec![0];
    for s in arr {
        let mut mask = 0;
        for ch in s.chars() {
            let mut ch = ch as usize;
            ch = ch as usize - 97;
            if (mask >> ch) & 1 != 0 {
                mask = 0;
                break;
            }
            mask |= 1 << ch;
        }
        if mask == 0 {
            continue;
        }
        let n = masks.len();
        for i in 0..n {
            let m = masks[i];
            if m & mask == 0 {
                masks.push(m | mask);
                ans = ans.max((((m | mask) as usize).count_ones()) as i32);
            }
        }
    }
    return ans;
}

fn main() {
    {
        let arr = vec!["un".to_string(), "iq".to_string(), "ue".to_string()];
        assert_eq!(max_length(arr), 4);
    }
    {
        let arr = vec![
            "cha".to_string(),
            "r".to_string(),
            "act".to_string(),
            "ers".to_string(),
        ];
        assert_eq!(max_length(arr), 6);
    }
    {
        let arr = vec!["abcdefghijklmnopqrstuvwxyz".to_string()];
        assert_eq!(max_length(arr), 26);
    }
}
