/*
 * @Date: 2021-07-23 18:50:09
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-07-23 19:03:05
 */

pub fn is_covered(ranges: Vec<Vec<i32>>, left: i32, right: i32) -> bool {
    let mut cover = vec![false; 51];
    for i in 0..ranges.len() {
        for j in ranges[i][0]..=ranges[i][1] {
            cover[j as usize] = true;
        }
    }
    for i in left..=right {
        if !cover[i as usize] {
            return false;
        }
    }
    true
}

fn main() {
  {
    let ranges = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
    let left = 2;
    let right = 5;
    assert!(is_covered(ranges, left, right));
  }
  {
    let ranges = vec![vec![10, 20], vec![10, 20]];
    let left = 21;
    let right = 21;
    assert!(!is_covered(ranges, left, right));
  }
}
