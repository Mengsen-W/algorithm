struct Solution;

impl Solution {
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
}

fn main() {
    let tests = vec![
        (
            vec![
                vec![0, 1, 1, 0],
                vec![0, 1, 1, 0],
                vec![1, 0, 0, 1],
                vec![1, 0, 0, 1],
            ],
            2,
        ),
        (vec![vec![0, 1], vec![1, 0]], 0),
        (vec![vec![1, 0], vec![1, 0]], -1),
    ];

    for (board, ans) in tests {
        assert_eq!(Solution::moves_to_chessboard(board), ans);
    }
}
