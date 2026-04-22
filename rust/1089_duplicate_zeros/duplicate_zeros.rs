/*
 * @Date: 2022-06-17 09:39:04
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-06-17 09:50:47
 * @FilePath: /algorithm/1089_duplicate_zeros/duplicate_zeros.rs
 */

pub fn duplicate_zeros(arr: &mut Vec<i32>) {
    let n = arr.len() as i32;

    let mut top = 0;
    let mut i = -1;
    while top < n {
        i += 1;
        if arr[i as usize] != 0 {
            top += 1;
        } else {
            top += 2;
        }
    }
    let mut j = n - 1;
    if top == n + 1 {
        arr[j as usize] = 0;
        j -= 1;
        i -= 1;
    }
    while j >= 0 {
        arr[j as usize] = arr[i as usize];
        j -= 1;
        if arr[i as usize] == 0 {
            arr[j as usize] = arr[i as usize];
            j -= 1;
        }
        i -= 1;
    }
}

fn main() {
    {
        let mut arr = vec![1, 0, 2, 3, 0, 4, 5, 0];
        duplicate_zeros(&mut arr);
        assert_eq!(arr, vec![1, 0, 0, 2, 3, 0, 0, 4]);
    }

    {
        let mut arr = vec![1, 2, 3];
        duplicate_zeros(&mut arr);
        assert_eq!(arr, vec![1, 2, 3]);
    }
}
