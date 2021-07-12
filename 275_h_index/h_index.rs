/*
 * @Date: 2021-07-12 08:06:47
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-07-12 08:23:05
 */

fn h_index(citations: Vec<i32>) -> i32 {
    let n: i32 = citations.len() as i32;
    let (mut left, mut right): (i32, i32) = (0, n - 1);
    while left <= right {
        let mid = left + (right - left) / 2;
        match citations[mid as usize] >= n - mid {
            true => right = mid - 1,
            false => left = mid + 1,
        }
    }
    n - left
}

fn main() {
    let citations = vec![0, 1, 3, 5, 6];
    assert_eq!(h_index(citations), 3);

    let citations = vec![1];
    assert_eq!(h_index(citations), 1);
}
