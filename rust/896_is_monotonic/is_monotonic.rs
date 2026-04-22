/*
 * @Author: Mengsen.Wang
 * @Date: 2021-02-28 14:59:25
 * @Last Modified by: Mengsen.Wang
 * @Last Modified time: 2021-02-28 15:10:47
 */

fn is_monotonic(a: Vec<i32>) -> bool {
    let mut inc = true;
    let mut dec = true;
    a.windows(2).all(|w| {
        if w[0] > w[1] {
            inc = false;
        }
        if w[0] < w[1] {
            dec = false;
        }
        inc | dec
    })
}

fn main() {
    let mut a: Vec<i32>;
    a = vec![1, 2, 2, 3];
    assert!(is_monotonic(a));
    a = vec![6, 5, 4, 4];
    assert!(is_monotonic(a));
    a = vec![1, 3, 2];
    assert!(!is_monotonic(a));
    a = vec![1, 2, 4, 5];
    assert!(is_monotonic(a));
    a = vec![1, 1, 1];
    assert!(is_monotonic(a));
}
