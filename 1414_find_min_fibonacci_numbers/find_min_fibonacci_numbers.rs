/*
 * @Date: 2022-02-03 14:07:07
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-02-03 14:48:35
 */

pub fn find_min_fibonacci_numbers(mut k: i32) -> i32 {
    let mut f: Vec<i32> = vec![1, 1];
    while f[f.len() - 1] < k {
        f.push(f[f.len() - 1] + f[f.len() - 2]);
    }
    let mut ans = 0;
    while k > 0 {
        let last = *f.last().unwrap();
        if k >= last {
            k -= last;
            ans += 1;
        }
        f.pop().unwrap();
    }
    ans
}

fn main() {
    assert_eq!(find_min_fibonacci_numbers(7), 2);
    assert_eq!(find_min_fibonacci_numbers(10), 2);
    assert_eq!(find_min_fibonacci_numbers(19), 3);
}
