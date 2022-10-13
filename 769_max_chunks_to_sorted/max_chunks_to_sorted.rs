/*
 * @Date: 2022-10-13
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-10-13
 * @FilePath: /algorithm/769_max_chunks_to_sorted/max_chunks_to_sorted.rs
 */

pub fn max_chunks_to_sorted(arr: Vec<i32>) -> i32 {
    let (mut m, mut res) = (0, 0);
    for i in 0..arr.len() {
        m = m.max(arr[i]);
        if m == i as i32 {
            res += 1;
        }
    }
    res
}

fn main() {
    {
        let arr = vec![4, 3, 2, 1, 0];
        let ans = 1;
        assert_eq!(max_chunks_to_sorted(arr), ans);
    }

    {
        let arr = vec![1, 0, 2, 3, 4];
        let ans = 4;
        assert_eq!(max_chunks_to_sorted(arr), ans);
    }
}
