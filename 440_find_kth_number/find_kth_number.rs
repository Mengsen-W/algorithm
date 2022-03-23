/*
 * @Date: 2022-03-23 00:37:38
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-03-23 01:15:04
 * @FilePath: /algorithm/440_find_kth_number/find_kth_number.rs
 */

pub fn find_kth_number(n: i32, k: i32) -> i32 {
    fn get_steps(curr: i32, n: i32) -> i32 {
        let n = n as i64;
        let mut steps = 0;
        let (mut first, mut last) = (curr as i64, curr as i64);
        while first <= n {
            steps += last.min(n) - first + 1;
            first = first * 10;
            last = last * 10 + 9;
        }
        steps as i32
    }
    let mut curr = 1;
    let mut k = k - 1;
    while k > 0 {
        let steps = get_steps(curr, n);
        if steps <= k {
            k -= steps;
            curr += 1;
        } else {
            curr = curr * 10;
            k -= 1;
        }
    }
    curr
}

fn main() {
    assert_eq!(find_kth_number(13, 2), 10);
    assert_eq!(find_kth_number(1, 1), 1);
    assert_eq!(find_kth_number(681692778, 351251360), 416126219);
}
