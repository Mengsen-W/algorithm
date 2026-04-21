/*
 * @Date: 2023-12-26
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-12-27
 * @FilePath: /algorithm/rust/1349_max_students/max_students.rs
 */

struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn max_students(seats: Vec<Vec<char>>) -> i32 {
        fn is_single_row_compliant(
            seats: &[Vec<char>],
            status: usize,
            n: usize,
            row: usize,
        ) -> bool {
            (0..n).all(|j| {
                if (status & (1 << j)) == 0 {
                    true
                } else {
                    seats[row][j] != '#' && (j == 0 || (status & (1 << (j - 1))) == 0)
                }
            })
        }

        fn is_cross_rows_compliant(status: usize, upper_row_status: usize, n: usize) -> bool {
            (0..n).all(|j| {
                if (status & (1 << j)) == 0 {
                    true
                } else {
                    (j == 0 || (upper_row_status & (1 << (j - 1))) == 0)
                        && (j == n - 1 || (upper_row_status & (1 << (j + 1))) == 0)
                }
            })
        }

        fn bit_count(mut bits: usize) -> i32 {
            bits = bits - ((bits >> 1) & 0x55555555);
            bits = (bits & 0x33333333) + ((bits >> 2) & 0x33333333);
            bits = (bits + (bits >> 4)) & 0x0f0f0f0f;
            bits = (bits + (bits >> 8)) & 0x00ff00ff;
            bits = (bits + (bits >> 16)) & 0x0000ffff;
            bits as i32
        }

        fn dp(
            seats: &Vec<Vec<char>>,
            row: usize,
            status: usize,
            n: usize,
            memo: &mut HashMap<i64, i32>,
        ) -> i32 {
            // let key = ((row << n) + status) as i64; 等同实现逻辑
            let key = ((row << n) | status) as i64;

            if !memo.contains_key(&key) {
                if !is_single_row_compliant(seats, status, n, row) {
                    memo.insert(key, i32::MIN);
                    return i32::MIN;
                }

                let students = bit_count(status);

                if row == 0 {
                    memo.insert(key, students);
                    return students;
                }

                let mut mx = 0;
                for upper_row_status in 0..(1 << n) {
                    if is_cross_rows_compliant(status, upper_row_status, n) {
                        mx = mx.max(dp(seats, row - 1, upper_row_status, n, memo));
                    }
                }
                memo.insert(key, students + mx);
            }
            *memo.get(&key).unwrap()
        }

        let mut mx = 0;
        let m = seats.len();
        let n = seats[0].len();
        let mut memo = HashMap::<i64, i32>::new();
        (0..(1 << n)).for_each(|i| mx = mx.max(dp(&seats, m - 1, i, n, &mut memo)));
        mx
    }
}

fn main() {
    let tests = vec![
        (
            vec![
                vec!['#', '.', '#', '#', '.', '#'],
                vec!['.', '#', '#', '#', '#', '.'],
                vec!['#', '.', '#', '#', '.', '#'],
            ],
            4,
        ),
        (
            vec![
                vec!['.', '#'],
                vec!['#', '#'],
                vec!['#', '.'],
                vec!['#', '#'],
                vec!['.', '#'],
            ],
            3,
        ),
        (
            vec![
                vec!['#', '.', '.', '.', '#'],
                vec!['.', '#', '.', '#', '.'],
                vec!['.', '.', '#', '.', '.'],
                vec!['.', '#', '.', '#', '.'],
                vec!['#', '.', '.', '.', '#'],
            ],
            10,
        ),
    ];

    for (seats, ans) in tests {
        assert_eq!(Solution::max_students(seats), ans);
    }
}
