/*
 * @Date: 2022-05-18 22:07:33
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-05-18 22:13:13
 * @FilePath: /algorithm/668_find_kth_number/find_kth_number.rs
 */

pub fn find_kth_number(m: i32, n: i32, k: i32) -> i32 {
    let mut l = 0;
    let mut r = m * n + 1;
    while l + 1 < r {
        let mid = l + (r - l) / 2;
        if (1..=m).map(|x| (mid / x).min(n)).sum::<i32>() < k {
            l = mid;
        } else {
            r = mid;
        }
    }
    r
}

fn main() {
    assert_eq!(find_kth_number(3, 3, 5), 3);
    assert_eq!(find_kth_number(2, 3, 6), 6);
}
