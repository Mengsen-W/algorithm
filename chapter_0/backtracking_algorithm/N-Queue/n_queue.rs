/*
 * @Author: Mengsen.Wang
 * @Date: 2021-01-20 16:55:04
 * @Last Modified by: Mengsen.Wang
 * @Last Modified time: 2021-01-20 18:04:06
 */

fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
    let mut res: Vec<Vec<char>> = vec![];
    let mut board: Vec<char> = vec![vec!['.', n]; n];
    backtrace(board, 0, res);
    res
}

fn backtrace(mut board: &Vec<char>, row: i32, mut res: &Vec<Vec<char>>) {
    if row == board.len() {
        res.push(board);
        return;
        // return true;
    }

    for col in 0..board[row].len() {
        if !isVaild(board, row, col) {
            continue;
        }
        // 做选择
        board[row][col] = 'Q';
        // 进入下一行决策
        backtrack(board, row + 1);
        // if backtrack(board, row + 1) {return true;}
        // 撤销选择
        board[row][col] = '.';
    }
    // return false;
}

fn is_vaild(board: &Vec<char>, row: i32, col: i32) -> bool {
    for i in 0..board.len() {
        if board[i][col] == 'Q' {
            return false;
        }
    }
    //TODO
}

fn main() {}
