/*
 * @Date: 2022-02-19 14:08:06
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-02-19 14:45:13
 */

pub fn pancake_sort(mut arr: Vec<i32>) -> Vec<i32> {
    let mut ans = vec![];
    for item in (2..=arr.len() as i32).rev() {
        let idx = arr.iter().position(|&x| x == item).unwrap() + 1;
        if item == idx as i32 {
            continue;
        }
        if idx > 1 {
            ans.push(idx as i32);
            arr[0..idx].reverse();
        }
        arr[0..item as usize].reverse();
        ans.push(item);
    }
    ans
}

fn main() {
    assert_eq!(pancake_sort(vec![3, 2, 4, 1]), vec![3, 4, 2, 3, 2]);
    assert_eq!(pancake_sort(vec![3, 2, 1]), vec![3]);
}
