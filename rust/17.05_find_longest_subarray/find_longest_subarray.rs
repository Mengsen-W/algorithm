/*
 * @Date: 2023-03-11
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-03-11
 * @FilePath: /algorithm/rust/17.05_find_longest_subarray/find_longest_subarray.rs
 */

pub fn find_longest_subarray(array: Vec<String>) -> Vec<String> {
    use std::collections::HashMap;
    let mut s = 0;
    let mut l = 0_i32;
    let mut r = 0_i32;
    let mut m = HashMap::new();
    m.insert(0, -1);

    array.iter().enumerate().for_each(|(i, x)| {
        if x.as_bytes().first().unwrap().is_ascii_alphabetic() {
            s += 1;
        } else {
            s -= 1;
        }

        if let Some(&v) = m.get(&s) {
            if i as i32 - v > r - l {
                l = v;
                r = i as i32;
            }
        } else {
            m.insert(s, i as i32);
        }
    });

    array[(l + 1) as usize..(r + 1) as usize].to_vec()
}

fn main() {
    {
        let array = vec![
            "A", "1", "B", "C", "D", "2", "3", "4", "E", "5", "F", "G", "6", "7", "H", "I", "J",
            "K", "L", "M",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        let ans: Vec<String> = vec![
            "A", "1", "B", "C", "D", "2", "3", "4", "E", "5", "F", "G", "6", "7",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        assert_eq!(find_longest_subarray(array), ans);
    }

    {
        let array = vec!["A", "A"].iter().map(|s| s.to_string()).collect();
        let ans: Vec<String> = Vec::new();
        assert_eq!(find_longest_subarray(array), ans);
    }
}
