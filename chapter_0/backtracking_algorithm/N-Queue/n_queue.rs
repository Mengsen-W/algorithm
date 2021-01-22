/*
 * @Author: Mengsen.Wang
 * @Date: 2021-01-20 16:55:04
 * @Last Modified by: Mengsen.Wang
 * @Last Modified time: 2021-01-20 18:04:06
 */

fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
    let mut res: Vec<Vec<String>> = vec![];
    let mut board: Vec<String> =
        vec![vec!['.'; n as usize].into_iter().collect::<String>(); n as usize];
    back_trace(&board, 0, &res);
    res
}

fn back_trace(mut board: &Vec<String>, row: i32, mut res: &Vec<Vec<String>>) {
    if row == board.len() as i32 {
        res.push(*board);
        return;
        // return true;
    }

    for col in 0..(*board[row as usize]).len() {
        if !is_valid(board, row, col as i32) {
            continue;
        }
        // 做选择
        board[row as usize].chars().nth(col as usize).unwrap() = 'Q';
        // 进入下一行决策
        back_trace(board, row + 1, res);
        // if backtrack(board, row + 1) {return true;}
        // 撤销选择
        board[row as usize].chars().nth(col as usize).unwrap() = '.';
    }
    // return false;
}

fn is_valid(board: &Vec<String>, row: i32, col: i32) -> bool {
    for i in 0..board.len() {
        if board[i as usize].chars().nth(col as usize).unwrap() == 'Q' {
            return false;
        }
    }

    let mut i: i32 = row - 1;
    let mut j: i32 = col + 1;
    while i >= 0 && j < board.len() as i32 {
        if board[i as usize].chars().nth(j as usize).unwrap() == 'Q' {
            return false;
        }
        i -= 1;
        j += 1;
    }

    let mut i: i32 = row - 1;
    let mut j: i32 = col - 1;
    while i >= 0 && j >= 0 {
        if board[i as usize].chars().nth(j as usize).unwrap() == 'Q' {
            return false;
        }
    }

    true
}

fn main() {
    let res = solve_n_queens(4);
}

#[test]
fn test() {
    let s: String = "Hello".to_string();
    println!("The result = {:?}", s.chars().nth(3).unwrap());
    println!("The result = {:?}", s.get(2..3).unwrap());
    let s: String = vec!['.'; 10].into_iter().collect::<String>();
    println!("The result = {:?}", s);
}
