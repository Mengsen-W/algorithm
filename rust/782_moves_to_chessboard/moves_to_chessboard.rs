/*
 * @Date: 2022-08-23
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-08-23
 * @FilePath: /algorithm/782_moves_to_chessboard/moves_to_chessboard.rs
 */

pub fn moves_to_chessboard(board: Vec<Vec<i32>>) -> i32 {
    let (n, mut row_cnt, mut col_cnt, mut row_swap, mut col_swap) = (board.len(), 0, 0, 0, 0);
    for i in 0..n {
        for j in 0..n {
            if (board[0][0] ^ board[i][0] ^ board[0][j] ^ board[i][j]) == 1 {
                return -1;
            }
        }
    }
    for i in 0..n {
        row_cnt += board[0][i];
        col_cnt += board[i][0];
        if board[i][0] as usize == i % 2 {
            row_swap += 1;
        }
        if board[0][i] as usize == i % 2 {
            col_swap += 1;
        }
    }
    if row_cnt as usize != n / 2 && row_cnt as usize != (n + 1) / 2 {
        return -1;
    }
    if col_cnt as usize != n / 2 && col_cnt as usize != (n + 1) / 2 {
        return -1;
    }
    if n % 2 == 1 {
        if row_swap % 2 == 1 {
            row_swap = n as i32 - row_swap;
        }
        if col_swap % 2 == 1 {
            col_swap = n as i32 - col_swap;
        }
    } else {
        row_swap = row_swap.min(n as i32 - row_swap);
        col_swap = col_swap.min(n as i32 - col_swap);
    }
    (row_swap + col_swap) / 2
}

fn main() {
    {
        let board = vec![
            vec![0, 1, 1, 0],
            vec![0, 1, 1, 0],
            vec![1, 0, 0, 1],
            vec![1, 0, 0, 1],
        ];
        let ans = 2;
        assert_eq!(moves_to_chessboard(board), ans);
    }

    {
        let board = vec![vec![0, 1], vec![1, 0]];
        let ans = 0;
        assert_eq!(moves_to_chessboard(board), ans);
    }

    {
        let board = vec![vec![1, 0], vec![1, 0]];
        let ans = -1;
        assert_eq!(moves_to_chessboard(board), ans);
    }
}
