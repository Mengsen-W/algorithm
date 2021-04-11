/*
 * @Date: 2021-04-11 09:55:31
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-04-11 10:11:56
 */

fn nth_ugly_number(n: i32) -> i32 {
    let n = n as usize;
    let mut ugly = vec![1; n];
    let mut idx = vec![0; 3];
    for i in 1..n {
        let a = ugly[idx[0]] * 2;
        let b = ugly[idx[1]] * 3;
        let c = ugly[idx[2]] * 5;
        let next = std::cmp::min(c, std::cmp::min(a, b));
        if next == a {
            idx[0] += 1;
        }
        if next == b {
            idx[1] += 1;
        }
        if next == c {
            idx[2] += 1;
        }
        ugly[i] = next;
    }

    ugly.pop().unwrap()
}

fn main() {
    assert_eq!(nth_ugly_number(10), 12);
    assert_eq!(nth_ugly_number(1), 1);
}
