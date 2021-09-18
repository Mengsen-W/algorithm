/*
 * @Date: 2021-09-17 08:40:51
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-09-17 08:58:09
 * @FilePath: /algorithm/36_is_valid_sudoku/is_valid_sudoku.rs
 * @Description: file content
 */

struct Solution;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut rows = [0; 9];
        let mut cols = [0; 9];
        let mut blks = [0; 9];
        for r in 0..9 {
            let row = &board[r];
            for c in 0..9 {
                let ch = row[c];
                if ch != '.' {
                    let mask = 1 << ch as i32 - '1' as i32;
                    let b = (r / 3) * 3 + (c / 3);
                    if rows[r] & mask != 0 || cols[c] & mask != 0 || blks[b] & mask != 0 {
                        return false;
                    }
                    rows[r] |= mask;
                    cols[c] |= mask;
                    blks[b] |= mask;
                }
            }
        }
        return true;
    }
}

fn main() {
    {
        let board: Vec<Vec<char>> = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        assert!(Solution::is_valid_sudoku(board));
    }
    {
        let board: Vec<Vec<char>> = vec![
            vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        assert!(!Solution::is_valid_sudoku(board));
    }
}
