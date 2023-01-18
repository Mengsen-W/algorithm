/*
 * @Date: 2022-06-08 09:41:52
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-06-08 09:44:27
 * @FilePath: /algorithm/1037_is_boomerang/is_boomerang.rs
 */

pub fn is_boomerang(points: Vec<Vec<i32>>) -> bool {
    (points[2][1] - points[1][1]) * (points[1][0] - points[0][0])
        != (points[1][1] - points[0][1]) * (points[2][0] - points[1][0])
}

fn main() {
    assert_eq!(is_boomerang(vec![vec![1, 1], vec![2, 3], vec![3, 2]]), true);
    assert_eq!(
        is_boomerang(vec![vec![1, 1], vec![2, 2], vec![3, 3]]),
        false
    );
}
