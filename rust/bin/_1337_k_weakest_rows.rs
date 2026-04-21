/*
 * @Date: 2021-08-01 15:01:26
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-08-01 17:30:59
 */

pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
    let mut arr: Vec<(usize, i32)> = mat
        .into_iter()
        .enumerate()
        .map(|(i, row)| {
            let (mut l, mut r) = (0, row.len());
            while l < r {
                let m = l + (r - l) / 2;
                match row[m] == 1 {
                    true => l = m + 1,
                    false => r = m,
                }
            }
            (l, i as i32)
        })
        .collect();
    arr.sort();
    arr.truncate(k as usize);
    arr.into_iter().map(|(_, i)| i).collect()
}

fn main() {
    {
        let mat = vec![
            vec![1, 1, 0, 0, 0],
            vec![1, 1, 1, 1, 0],
            vec![1, 0, 0, 0, 0],
            vec![1, 1, 0, 0, 0],
            vec![1, 1, 1, 1, 1],
        ];
        let k = 3;
        let ans = vec![2, 0, 3];
        assert_eq!(k_weakest_rows(mat, k), ans);
    }
    {
        let mat = vec![
            vec![1, 0, 0, 0],
            vec![1, 1, 1, 1],
            vec![1, 0, 0, 0],
            vec![1, 0, 0, 0],
        ];
        let k = 2;
        let ans = vec![0, 2];
        assert_eq!(k_weakest_rows(mat, k), ans);
    }
}
