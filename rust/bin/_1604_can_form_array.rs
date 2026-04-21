/*
 * @Date: 2022-09-22
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-09-22
 * @FilePath: /algorithm/1604_can_form_array/can_form_array.rs
 */

pub fn can_form_array(arr: Vec<i32>, mut pieces: Vec<Vec<i32>>) -> bool {
    use std::collections::HashMap;
    let indices = arr
        .iter()
        .enumerate()
        .map(|(i, n)| (n, i))
        .collect::<HashMap<_, _>>();
    pieces.sort_unstable_by_key(|piece| indices.get(&piece[0]));
    arr == pieces.concat()
}

fn main() {
    {
        let arr = vec![15, 88];
        let pieces = vec![vec![88], vec![15]];
        assert!(can_form_array(arr, pieces));
    }

    {
        let arr = vec![49, 18, 16];
        let pieces = vec![vec![16, 18, 49]];
        assert!(!can_form_array(arr, pieces));
    }

    {
        let arr = vec![91, 4, 64, 78];
        let pieces = vec![vec![78], vec![4, 64], vec![91]];
        assert!(can_form_array(arr, pieces));
    }
}
