/*
 * @Author: Mengsen.Wang
 * @Date: 2021-03-02 14:56:36
 * @Last Modified by: Mengsen.Wang
 * @Last Modified time: 2021-03-02 15:13:53
 */

struct NumMatrix(Vec<Vec<i32>>);

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumMatrix {
    fn new(mut matrix: Vec<Vec<i32>>) -> Self {
        if matrix.len() == 0 {
            Self(vec![vec![0]])
        } else {
            matrix.iter_mut().for_each(|i| {
                i.iter_mut().fold(0, |s, x| {
                    *x += s;
                    *x
                });
            });
            let s = vec![0; matrix[0].len()];
            matrix.iter_mut().fold(s, |s, x| {
                s.iter().zip(x.iter_mut()).for_each(|(s, x)| *x += s);
                x.to_vec()
            });
            Self(matrix)
        }
    }

    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        self.0[row2 as usize][col2 as usize]
            - if col1 > 0 {
                self.0[row2 as usize][(col1 - 1) as usize]
            } else {
                0
            }
            - if row1 > 0 {
                self.0[(row1 - 1) as usize][col2 as usize]
            } else {
                0
            }
            + if col1 * row1 > 0 {
                self.0[(row1 - 1) as usize][(col1 - 1) as usize]
            } else {
                0
            }
    }
}

fn main() {
    let obj = NumMatrix::new(
        vec![
            vec![3, 0, 1, 4, 2],
            vec![5, 6, 3, 2, 1],
            vec![1, 2, 0, 1, 5],
            vec![4, 1, 0, 1, 7],
        ],
        vec![1, 0, 3, 0, 5],
    );
    assert_eq!(8, obj.sum_region(2, 1, 4, 3));
    assert_eq!(11, obj.sum_region(1, 1, 2, 2));
    assert_eq!(12, obj.sum_region(1, 2, 2, 4));
}
