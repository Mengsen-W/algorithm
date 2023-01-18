/*
 * @Date: 2022-04-22 09:20:03
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-04-22 09:26:51
 * @FilePath: /algorithm/396_max_rotate_function/max_rotate_function.rs
 */

pub fn max_rotate_function(a: Vec<i32>) -> i32 {
    let sum: i32 = a.iter().sum();
    let n = a.len();
    let mut func = a
        .iter()
        .enumerate()
        .fold(0, |acc, (i, t)| acc + i as i32 * t);
    let mut ans = func;

    for &i in a.iter().rev() {
        func += sum - n as i32 * i;
        if func > ans {
            ans = func;
        }
    }
    ans
}

fn main() {
    assert_eq!(max_rotate_function(vec![4, 3, 2, 6]), 26);
    assert_eq!(max_rotate_function(vec![100]), 0);
}
