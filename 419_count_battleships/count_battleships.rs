/*
 * @Date: 2021-12-18 01:04:35
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-12-18 01:38:13
 */

pub fn count_battleships(board: Vec<Vec<char>>) -> i32 {
    let mut ans = 0;
    for (index1, row) in board.iter().enumerate() {
        for (index2, ch) in row.iter().enumerate() {
            if ch == &'X'
                && !(index1 > 0 && board[index1 - 1][index2] == 'X')
                && !(index2 > 0 && board[index1][index2 - 1] == 'X')
            {
                ans += 1;
            }
        }
    }
    ans
}

fn main() {
    assert_eq!(
        count_battleships(vec![
            vec!['X', '.', '.', 'X'],
            vec!['.', '.', '.', 'X'],
            vec!['.', '.', '.', 'X'],
        ]),
        2
    );

    assert_eq!(count_battleships(vec![vec!['.'],]), 0);
}
