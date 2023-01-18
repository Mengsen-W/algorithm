/*
 * @Date: 2022-11-29
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-11-29
 * @FilePath: /algorithm/1758_min_operations/min_operations.rs
 */

pub fn min_operations(s: String) -> i32 {
    let mut cnt = 0;
    let s: Vec<usize> = s.chars().map(|c| c as usize).collect();
    for i in 0..s.len() {
        let c = s[i];
        // 48 == '0'
        if c != (48 + i % 2) {
            cnt += 1;
        }
    }
    cnt.min(s.len() as i32 - cnt)
}

fn main() {
    assert_eq!(min_operations(String::from("0100")), 1);
    assert_eq!(min_operations(String::from("10")), 0);
    assert_eq!(min_operations(String::from("1111")), 2);
}
