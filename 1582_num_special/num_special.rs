/*
 * @Date: 2022-09-04
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-09-04
 * @FilePath: /algorithm/1582_num_special/num_special.rs
 */

pub fn num_special(mat: Vec<Vec<i32>>) -> i32 {
    let mut mat = mat;
    let (m, n) = (mat.len(), mat.first().unwrap_or(&vec![]).len());
    (0..m).for_each(|i| {
        let mut cnt1 = 0;
        (0..n).for_each(|j| {
            if mat[i][j] == 1 {
                cnt1 += 1;
            }
        });
        if i == 0 {
            cnt1 -= 1;
        }
        if cnt1 > 0 {
            (0..n).for_each(|j| {
                if mat[i][j] == 1 {
                    mat[0][j] += cnt1;
                }
            })
        }
    });
    (0..n).fold(0, |mut sum, i| {
        if mat[0][i] == 1 {
            sum += 1;
        }
        sum
    })
}

fn main() {
    assert_eq!(
        num_special(
            [[1, 0, 0], [0, 0, 1], [1, 0, 0]]
                .iter()
                .map(|v| v.to_vec())
                .collect()
        ),
        1
    );

    assert_eq!(
        num_special(
            [[1, 0, 0], [0, 1, 0], [0, 0, 1]]
                .iter()
                .map(|v| v.to_vec())
                .collect()
        ),
        3
    );

    assert_eq!(
        num_special(
            [[0, 0, 0, 1], [1, 0, 0, 0], [0, 1, 1, 0], [0, 0, 0, 0]]
                .iter()
                .map(|v| v.to_vec())
                .collect()
        ),
        2
    );
    assert_eq!(
        num_special(
            [
                [0, 0, 0, 0, 0],
                [1, 0, 0, 0, 0],
                [0, 1, 0, 0, 0],
                [0, 0, 1, 0, 0],
                [0, 0, 0, 1, 1]
            ]
            .iter()
            .map(|v| v.to_vec())
            .collect()
        ),
        3
    );
}
