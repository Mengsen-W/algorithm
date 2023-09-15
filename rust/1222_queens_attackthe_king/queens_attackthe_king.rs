/*
 * @Date: 2023-09-15
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-09-15
 * @FilePath: /algorithm/rust/1222_queens_attackthe_king/queens_attackthe_king.rs
 */

struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn queens_attackthe_king(queens: Vec<Vec<i32>>, king: Vec<i32>) -> Vec<Vec<i32>> {
        #[inline]
        fn is_valid(x: i32, y: i32) -> bool {
            x < 8 && y < 8 && x >= 0 && y >= 0
        }

        let mut ans = vec![];

        let x_king = king[0];
        let y_king = king[1];

        let queen_set: HashSet<(i32, i32)> = queens.into_iter().map(|q| (q[0], q[1])).collect();

        let dirs: [(i32, i32); 8] = [
            (0, -1),
            (1, -1),
            (1, 0),
            (1, 1),
            (0, 1),
            (-1, 1),
            (-1, 0),
            (-1, -1),
        ];

        for (dx, dy) in dirs.iter() {
            let mut new_x = x_king + dx;
            let mut new_y = y_king + dy;
            while is_valid(new_x, new_y) {
                if queen_set.contains(&(new_x, new_y)) {
                    ans.push(vec![new_x, new_y]);
                    break;
                }
                new_x += dx;
                new_y += dy;
            }
        }

        ans
    }
}

fn main() {
    let tests = vec![
        (
            vec![
                vec![0, 1],
                vec![1, 0],
                vec![4, 0],
                vec![0, 4],
                vec![3, 3],
                vec![2, 4],
            ],
            vec![0, 0],
            vec![vec![1, 0], vec![3, 3], vec![0, 1]],
        ),
        (
            vec![
                vec![0, 0],
                vec![1, 1],
                vec![2, 2],
                vec![3, 4],
                vec![3, 5],
                vec![4, 4],
                vec![4, 5],
            ],
            vec![3, 3],
            vec![vec![4, 4], vec![3, 4], vec![2, 2]],
        ),
        (
            vec![
                vec![5, 6],
                vec![7, 7],
                vec![2, 1],
                vec![0, 7],
                vec![1, 6],
                vec![5, 1],
                vec![3, 7],
                vec![0, 3],
                vec![4, 0],
                vec![1, 2],
                vec![6, 3],
                vec![5, 0],
                vec![0, 4],
                vec![2, 2],
                vec![1, 1],
                vec![6, 4],
                vec![5, 4],
                vec![0, 0],
                vec![2, 6],
                vec![4, 5],
                vec![5, 2],
                vec![1, 4],
                vec![7, 5],
                vec![2, 3],
                vec![0, 5],
                vec![4, 2],
                vec![1, 0],
                vec![2, 7],
                vec![0, 1],
                vec![4, 6],
                vec![6, 1],
                vec![0, 6],
                vec![4, 3],
                vec![1, 7],
            ],
            vec![3, 4],
            vec![
                vec![4, 3],
                vec![5, 4],
                vec![4, 5],
                vec![3, 7],
                vec![1, 6],
                vec![1, 4],
                vec![2, 3],
            ],
        ),
    ];

    for (queens, king, ans) in tests {
        assert_eq!(Solution::queens_attackthe_king(queens, king), ans);
    }
}
