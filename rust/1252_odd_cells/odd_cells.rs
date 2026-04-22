/*
 * @Date: 2022-07-12
 * @LastEditors: mengsenwang mengsen_wang@163.com
 * @LastEditTime: 2022-07-12
 * @FilePath: /algorithm/1252_odd_cells/odd_cells.rs
 */

pub fn odd_cells(m: i32, n: i32, indices: Vec<Vec<i32>>) -> i32 {
    let (m, n) = (m as usize, n as usize);

    let (mut rows, mut cols) = (vec![0; m], vec![0; n]);
    for index in indices {
        rows[index[0] as usize] += 1;
        cols[index[1] as usize] += 1;
    }
    let (mut oddx, mut oddy) = (0, 0);
    for i in 0..m {
        if (rows[i] & 1) != 0 {
            oddx += 1;
        }
    }

    for i in 0..n {
        if (cols[i] & 1) != 0 {
            oddy += 1;
        }
    }
    (oddx * (n - oddy) + (m - oddx) * oddy) as i32
}

fn main() {
    assert_eq!(odd_cells(2, 3, vec![vec![0, 1], vec![1, 1]]), 6);
    assert_eq!(odd_cells(2, 2, vec![vec![1, 1], vec![0, 0]]), 0);
}
