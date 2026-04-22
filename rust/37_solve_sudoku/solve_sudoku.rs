struct Solution;

use std::collections::BinaryHeap;

impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let mut row_has = [[false; 9]; 9]; // row_has[i][x] 表示 i 行是否有数字 x
        let mut col_has = [[false; 9]; 9]; // col_has[j][x] 表示 j 列是否有数字 x
        let mut sub_box_has = [[[false; 9]; 3]; 3]; // sub_box_has[i'][j'][x] 表示 (i',j') 宫是否有数字 x
        let mut empty_pos = vec![]; // 空格子的位置

        for (i, row) in board.iter().enumerate() {
            for (j, &b) in row.iter().enumerate() {
                if b == '.' {
                    empty_pos.push((i, j)); // 记录空格子的位置
                } else {
                    let x = (b as u8 - b'1') as usize; // 字符 '1'~'9' 转成数字 0~8
                                                       // 标记行、列、宫包含数字 x
                    row_has[i][x] = true;
                    col_has[j][x] = true;
                    sub_box_has[i / 3][j / 3][x] = true;
                }
            }
        }

        // 计算 (i, j) 这个空格子的待定数字个数
        let get_candidates = |i: usize, j: usize| -> i8 {
            let mut candidates = 9;
            for x in 0..9 {
                if row_has[i][x] || col_has[j][x] || sub_box_has[i / 3][j / 3][x] {
                    candidates -= 1;
                }
            }
            candidates
        };

        let mut empty_heap = BinaryHeap::new();
        for (i, j) in empty_pos {
            // 取相反数，把 empty_pq 当作最小堆
            empty_heap.push((-get_candidates(i, j), i, j));
        }

        // 每次递归，选一个空格子，枚举填入的数字
        fn dfs(
            board: &mut [Vec<char>],
            row_has: &mut [[bool; 9]; 9],
            col_has: &mut [[bool; 9]; 9],
            sub_box_has: &mut [[[bool; 9]; 3]; 3],
            empty_heap: &mut BinaryHeap<(i8, usize, usize)>,
        ) -> bool {
            if empty_heap.is_empty() {
                // 所有格子都已填入数字
                return true; // 完成数独
            }

            // 数独玩法：优先考虑待定数字个数最少的空格子
            let (_, i, j) = empty_heap.pop().unwrap();

            let mut candidates = 0; // 受之前填入的数字影响，实际待定数字个数可能比入堆时的少，需要重新计算
                                    // 枚举没填过的数字 x，填入 board[i][j]
            for x in 0..9 {
                if row_has[i][x] || col_has[j][x] || sub_box_has[i / 3][j / 3][x] {
                    continue; // x 填过了
                }

                // 填入 board[i][j]
                board[i][j] = (b'1' + x as u8) as char;
                // 标记行、列、宫包含数字 x
                row_has[i][x] = true;
                col_has[j][x] = true;
                sub_box_has[i / 3][j / 3][x] = true;

                // 填下一个空格子
                if dfs(board, row_has, col_has, sub_box_has, empty_heap) {
                    return true; // 完成数独
                }

                // 恢复现场（撤销）
                // 注意 board[i][j] 无需恢复现场，因为我们会直接覆盖掉之前填入的数字
                row_has[i][x] = false;
                col_has[j][x] = false;
                sub_box_has[i / 3][j / 3][x] = false;

                // 统计待定数字个数
                candidates += 1;
            }

            // 恢复现场（撤销）
            empty_heap.push((-candidates, i, j)); // 重新入堆（更新待定数字个数）
                                                  // 所有填法都不行，说明之前（祖先节点）的填法是错的
            false
        }

        dfs(
            board,
            &mut row_has,
            &mut col_has,
            &mut sub_box_has,
            &mut empty_heap,
        );
    }
}

fn main() {
    let tests = vec![(
        vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ],
        vec![
            vec!['5', '3', '4', '6', '7', '8', '9', '1', '2'],
            vec!['6', '7', '2', '1', '9', '5', '3', '4', '8'],
            vec!['1', '9', '8', '3', '4', '2', '5', '6', '7'],
            vec!['8', '5', '9', '7', '6', '1', '4', '2', '3'],
            vec!['4', '2', '6', '8', '5', '3', '7', '9', '1'],
            vec!['7', '1', '3', '9', '2', '4', '8', '5', '6'],
            vec!['9', '6', '1', '5', '3', '7', '2', '8', '4'],
            vec!['2', '8', '7', '4', '1', '9', '6', '3', '5'],
            vec!['3', '4', '5', '2', '8', '6', '1', '7', '9'],
        ],
    )];

    for (mut board, ans) in tests {
        Solution::solve_sudoku(&mut board);
        assert_eq!(board, ans);
    }
}
