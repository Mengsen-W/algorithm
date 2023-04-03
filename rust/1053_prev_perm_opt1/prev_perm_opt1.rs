/*
 * @Date: 2023-04-03
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-04-03
 * @FilePath: /algorithm/rust/1053_prev_perm_opt1/prev_perm_opt1.rs
 */

pub fn prev_perm_opt1(mut arr: Vec<i32>) -> Vec<i32> {
    let n = arr.len();
    for i in (0..=(n - 2)).rev() {
        if arr[i] > arr[i + 1] {
            let mut j = n - 1;
            while arr[j] >= arr[i] || arr[j] == arr[j - 1] {
                j -= 1;
            }
            arr.swap(i, j);
            break;
        }
    }
    arr
}

fn main() {
    {
        let arr = vec![3, 2, 1];
        let ans = vec![3, 1, 2];
        assert_eq!(prev_perm_opt1(arr), ans);
    }

    {
        let arr = vec![1, 1, 5];
        let ans = vec![1, 1, 5];
        assert_eq!(prev_perm_opt1(arr), ans);
    }

    {
        let arr = vec![1, 9, 4, 6, 7];
        let ans = vec![1, 7, 4, 6, 9];
        assert_eq!(prev_perm_opt1(arr), ans);
    }
}
