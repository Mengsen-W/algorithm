/*
 * @Date: 2022-03-13 00:28:29
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-03-13 00:42:47
 * @FilePath: /algorithm/393_valid_utf8/valid_utf8.rs
 */

struct Solution;

impl Solution {
    pub fn valid_utf8(data: Vec<i32>) -> bool {
        let data: Vec<u8> = data.into_iter().map(|x| (x & 0xff) as u8).collect();
        let mut pos = 0;
        while pos < data.len() {
            if let Some(n) = Self::check(&data, pos) {
                pos += n;
            } else {
                return false;
            }
        }
        true
    }

    fn check(data: &Vec<u8>, i: usize) -> Option<usize> {
        let n = if data[i] & 0b10000000 == 0b00000000 {
            1
        } else if data[i] & 0b11100000 == 0b11000000 {
            2
        } else if data[i] & 0b11110000 == 0b11100000 {
            3
        } else if data[i] & 0b11111000 == 0b11110000 {
            4
        } else {
            return None;
        };
        if (1..n).all(|j| i + j < data.len() && data[i + j] & 0b11000000 == 0b10000000) {
            Some(n)
        } else {
            None
        }
    }
}

fn main() {
    assert_eq!(Solution::valid_utf8(vec![197, 130, 1]), true);
    assert_eq!(Solution::valid_utf8(vec![235, 140, 4]), false);
}
