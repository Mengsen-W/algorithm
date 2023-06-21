/*
 * @Date: 2023-06-21
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-06-21
 * @FilePath: /algorithm/rust/LCP_41_flip_chess/flip_chess.rs
 */

struct Solution;

const DIRS: [(i32, i32); 8] = [
    (1, 0),
    (-1, 0),
    (0, 1),
    (0, -1),
    (1, 1),
    (1, -1),
    (-1, 1),
    (-1, -1),
];

impl Solution {
    pub fn flip_chess(chessboard: Vec<String>) -> i32 {
        let n = chessboard.len();
        let m = chessboard[0].len();
        let mut board = chessboard
            .into_iter()
            .map(|s| s.chars().collect())
            .collect::<Vec<Vec<char>>>();
        let mut ans = 0;

        for i in 0..n {
            for j in 0..m {
                if board[i][j] == '.' {
                    board[i][j] = 'X';
                    let mut paths = vec![];
                    ans = ans.max(Self::dfs(&mut board, &mut paths, i, j, 0));
                    paths.iter().for_each(|&(r, c)| board[r][c] = 'O');
                    board[i][j] = '.';
                }
            }
        }

        ans as i32
    }

    fn dfs(
        board: &mut Vec<Vec<char>>,
        paths: &mut Vec<(usize, usize)>,
        i: usize,
        j: usize,
        cur: usize,
    ) -> usize {
        let n = board.len();
        let m = board[0].len();

        for &(dx, dy) in &DIRS {
            let k = paths.len();
            let mut x = (i as i32 + dx) as usize;
            let mut y = (j as i32 + dy) as usize;

            while x < n && y < m && board[x][y] == 'O' {
                paths.push((x, y));
                x = (x as i32 + dx) as usize;
                y = (y as i32 + dy) as usize;
            }

            if x < n && y < m && board[x][y] == 'X' {
                for t in cur..paths.len() {
                    let (tx, ty) = paths[t];
                    board[tx][ty] = 'X';
                }
            } else {
                paths.drain(k..);
            }
        }

        for t in cur..paths.len() {
            let (tx, ty) = paths[t];
            Self::dfs(board, paths, tx, ty, paths.len());
        }

        paths.len()
    }
}

fn main() {
    {
        let chessboard = vec!["....X.", "....X.", "XOOO..", "......", "......"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let ans = 3;
        assert_eq!(Solution::flip_chess(chessboard), ans);
    }

    {
        let chessboard = vec![".X.", ".O.", "XO."]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let ans = 2;
        assert_eq!(Solution::flip_chess(chessboard), ans);
    }

    {
        let chessboard = vec![
            ".......", ".......", ".......", "X......", ".O.....", "..O....", "....OOX",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        let ans = 4;
        assert_eq!(Solution::flip_chess(chessboard), ans);
    }
}
