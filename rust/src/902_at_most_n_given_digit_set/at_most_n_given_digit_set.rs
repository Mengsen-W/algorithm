/*
 * @Date: 2022-10-18
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-10-18
 * @FilePath: /algorithm/902_at_most_n_given_digit_set/at_most_n_given_digit_set.rs
 */

pub fn at_most_n_given_digit_set(digits: Vec<String>, n: i32) -> i32 {
    let s: Vec<char> = n.to_string().chars().collect();
    let digits: Vec<Vec<char>> = digits.iter().map(|s| s.chars().collect()).collect();
    let m = digits.len();
    let k = s.len();
    let mut bits: Vec<i32> = vec![];
    let mut is_limit = true;
    for i in 0..k {
        if !is_limit {
            bits.push(m as i32 - 1);
        } else {
            let mut select_index: i32 = -1;
            for j in 0..m {
                if digits[j][0] <= s[i] {
                    select_index = j as i32;
                } else {
                    break;
                }
            }
            if select_index >= 0 {
                bits.push(select_index);
                if digits[select_index as usize][0] < s[i] {
                    is_limit = false;
                }
            } else {
                let mut len = bits.len() as i32;
                while !bits.is_empty() && bits.last() == Some(&0) {
                    bits.pop();
                }
                if !bits.is_empty() {
                    *(bits.last_mut().unwrap()) -= 1;
                } else {
                    len -= 1;
                }
                while bits.len() as i32 <= len {
                    bits.push(m as i32 - 1);
                }
                is_limit = false;
            }
        }
    }
    let mut ans: i32 = 0;
    for i in 0..bits.len() {
        ans = ans * m as i32 + (bits[i] + 1);
    }
    return ans;
}

fn main() {
    {
        let digits = vec!["1", "3", "5", "7"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let n = 100;
        let ans = 20;
        assert_eq!(at_most_n_given_digit_set(digits, n), ans);
    }

    {
        let digits = vec!["1", "4", "9"].iter().map(|s| s.to_string()).collect();
        let n = 1000000000;
        let ans = 29523;
        assert_eq!(at_most_n_given_digit_set(digits, n), ans);
    }

    {
        let digits = vec!["7"].iter().map(|s| s.to_string()).collect();
        let n = 8;
        let ans = 1;
        assert_eq!(at_most_n_given_digit_set(digits, n), ans);
    }

    {
        let digits = vec!["9"].iter().map(|s| s.to_string()).collect();
        let n = 35;
        let ans = 1;
        assert_eq!(at_most_n_given_digit_set(digits, n), ans);
    }
}
