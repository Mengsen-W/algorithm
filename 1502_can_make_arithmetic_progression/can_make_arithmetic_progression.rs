/*
 * @Author: Mengsen.Wang
 * @Date: 2021-02-18 09:45:24
 * @Last Modified by: Mengsen.Wang
 * @Last Modified time: 2021-02-18 09:49:03
 */

fn can_make_arithmetic_progression(mut arr: Vec<i32>) -> bool {
    arr.sort_unstable();
    let diff = arr[1] - arr[0];
    let size = arr.len();
    for i in 2..size {
        if (arr[i] - arr[i - 1]) != diff {
            return false;
        }
    }
    true
}

fn main() {
    let mut arr: Vec<i32>;
    arr = vec![3, 5, 1];
    assert!(can_make_arithmetic_progression(arr.clone()));
    arr = vec![1, 2, 4];
    assert!(!can_make_arithmetic_progression(arr.clone()));
}
