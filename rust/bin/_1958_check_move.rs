struct Solution;

impl Solution {
    pub fn check_move(board: Vec<Vec<char>>, r_move: i32, c_move: i32, color: char) -> bool {
        let m = board.len() as i32;
        let n = board[0].len() as i32;
        for (dx, dy) in [
            (1, 0),
            (1, 1),
            (0, 1),
            (-1, 1),
            (-1, 0),
            (-1, -1),
            (0, -1),
            (1, -1),
        ] {
            let mut x = r_move + dx;
            let mut y = c_move + dy;
            if x < 0
                || x >= m
                || y < 0
                || y >= n
                || board[x as usize][y as usize] == '.'
                || board[x as usize][y as usize] == color
            {
                continue;
            }
            loop {
                x += dx;
                y += dy;
                if x < 0 || x >= m || y < 0 || y >= n || board[x as usize][y as usize] == '.' {
                    break;
                }
                if board[x as usize][y as usize] == color {
                    return true;
                }
            }
        }
        false
    }
}

fn main() {
    let tests = vec![
        (
            vec![
                vec!['.', '.', '.', 'B', '.', '.', '.', '.'],
                vec!['.', '.', '.', 'W', '.', '.', '.', '.'],
                vec!['.', '.', '.', 'W', '.', '.', '.', '.'],
                vec!['.', '.', '.', 'W', '.', '.', '.', '.'],
                vec!['W', 'B', 'B', '.', 'W', 'W', 'W', 'B'],
                vec!['.', '.', '.', 'B', '.', '.', '.', '.'],
                vec!['.', '.', '.', 'B', '.', '.', '.', '.'],
                vec!['.', '.', '.', 'W', '.', '.', '.', '.'],
            ],
            4,
            3,
            'B',
            true,
        ),
        (
            vec![
                vec!['.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', 'B', '.', '.', 'W', '.', '.', '.'],
                vec!['.', '.', 'W', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', 'W', 'B', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', 'B', 'W', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', 'W', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', 'B'],
            ],
            4,
            4,
            'W',
            false,
        ),
    ];

    for (board, r_move, c_move, color, ans) in tests {
        assert_eq!(Solution::check_move(board, r_move, c_move, color), ans);
    }
}
