/*
 * @Author: Mengsen.Wang
 * @Date: 2021-02-19 09:02:22
 * @Last Modified by: Mengsen.Wang
 * @Last Modified time: 2021-02-19 09:06:48
 */

fn longest_ones(a: Vec<i32>, k: i32) -> i32 {
    let mut left = 0;
    let mut right = 0;
    let mut count = 0;
    let size = a.len();

    while right < size {
        if a[right] == 0 {
            count += 1;
        }
        right += 1;
        if count > k {
            if a[left] == 0 {
                count -= 1;
            }
            left += 1;
        }
    }
    (right - left) as i32
}

fn main() {
    let mut a: Vec<i32>;
    a = vec![1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0];
    assert_eq!(longest_ones(a, 2), 6);
    a = vec![0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1];
    assert_eq!(longest_ones(a, 3), 10);
}
