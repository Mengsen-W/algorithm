/*
 * @Date: 2021-11-29 03:24:48
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-11-29 04:55:33
 */

pub fn kth_smallest_prime_fraction(arr: Vec<i32>, k: i32) -> Vec<i32> {
    let n = arr.len();
    let (mut left, mut right) = (0.0, 1.0);
    loop {
        let mid = (left + right) / 2.0;
        let (mut i, mut count): (i32, i32) = (-1, 0);
        let (mut x, mut y) = (0, 1);

        for j in 1..n {
            while arr[(i + 1) as usize] as f64 / (arr[j] as f64) < mid {
                i += 1;
                if arr[i as usize] * y > arr[j] * x {
                    x = arr[i as usize];
                    y = arr[j];
                }
            }
            count += i + 1;
        }
        if count == k {
            return vec![x as i32, y as i32];
        }

        if count < k {
            left = mid;
        } else {
            right = mid;
        }
    }
}

fn main() {
    assert_eq!(kth_smallest_prime_fraction(vec![1, 2, 3, 5], 3), vec![2, 5]);
    assert_eq!(kth_smallest_prime_fraction(vec![1, 7], 1), vec![1, 7]);
}
