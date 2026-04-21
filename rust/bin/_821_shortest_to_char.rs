/*
 * @Date: 2022-04-19 07:19:13
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-04-19 07:26:47
 * @FilePath: /algorithm/821_shortest_to_char/shortest_to_char.rs
 */

pub fn shortest_to_char(s: String, c: char) -> Vec<i32> {
    let cmp = |dist: &mut i32, e: char| {
        if e == c {
            *dist = 0;
        } else {
            *dist += 1;
        }
        Some(*dist)
    };
    // 从后往前扫描
    let mut b: Vec<_> = s.chars().rev().scan(s.len() as i32, cmp).collect();
    s.chars()
        .scan(s.len() as i32, cmp) // 从前往后扫描
        .zip(b.drain(..).rev()) // zip 前面的结果的倒序，
        .map(|e| e.0.min(e.1)) // 逐个一一比较
        .collect()
}

fn main() {
    assert_eq!(
        shortest_to_char("loveleetcode".to_string(), 'e'),
        vec![3, 2, 1, 0, 1, 0, 0, 1, 2, 2, 1, 0]
    );
    assert_eq!(shortest_to_char("aaab".to_string(), 'b'), vec![3, 2, 1, 0]);
}
