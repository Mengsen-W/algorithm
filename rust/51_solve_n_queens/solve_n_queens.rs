struct Solution;

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let mut solutions = Vec::new();
        let mut queens = vec![-1; n as usize];
        let mut columns = std::collections::HashSet::new();
        let mut diagonal1 = std::collections::HashSet::new();
        let mut diagonal2 = std::collections::HashSet::new();
        let row = vec![".".to_string(); n as usize];

        fn generate_board(queens: &Vec<i32>, n: usize, row: &Vec<String>) -> Vec<String> {
            let mut board = Vec::new();
            for &q in queens.iter() {
                let mut r = row.clone();
                r[q as usize] = "Q".to_string();
                board.push(r.join(""));
            }
            board
        }

        fn backtrack(
            row: usize,
            n: usize,
            queens: &mut Vec<i32>,
            columns: &mut std::collections::HashSet<usize>,
            diagonal1: &mut std::collections::HashSet<i32>,
            diagonal2: &mut std::collections::HashSet<i32>,
            solutions: &mut Vec<Vec<String>>,
            row_pattern: &Vec<String>,
        ) {
            if row == n {
                let board = generate_board(queens, n, row_pattern);
                solutions.push(board);
            } else {
                for i in 0..n {
                    if columns.contains(&i)
                        || diagonal1.contains(&(row as i32 - i as i32))
                        || diagonal2.contains(&(row as i32 + i as i32))
                    {
                        continue;
                    }
                    queens[row] = i as i32;
                    columns.insert(i);
                    diagonal1.insert(row as i32 - i as i32);
                    diagonal2.insert(row as i32 + i as i32);
                    backtrack(
                        row + 1,
                        n,
                        queens,
                        columns,
                        diagonal1,
                        diagonal2,
                        solutions,
                        row_pattern,
                    );
                    columns.remove(&i);
                    diagonal1.remove(&(row as i32 - i as i32));
                    diagonal2.remove(&(row as i32 + i as i32));
                }
            }
        }

        backtrack(
            0,
            n as usize,
            &mut queens,
            &mut columns,
            &mut diagonal1,
            &mut diagonal2,
            &mut solutions,
            &row,
        );
        solutions
    }
}

fn main() {
    let tests = vec![
        (
            4,
            vec![
                vec![".Q..", "...Q", "Q...", "..Q."],
                vec!["..Q.", "Q...", "...Q", ".Q.."],
            ],
        ),
        (1, vec![vec!["Q"]]),
    ];

    for (n, ans) in tests {
        assert_eq!(Solution::solve_n_queens(n), ans);
    }
}
