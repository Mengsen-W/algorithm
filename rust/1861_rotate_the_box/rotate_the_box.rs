struct Solution;

impl Solution {
    pub fn rotate_the_box(box_grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
        let m = box_grid.len();
        let n = box_grid[0].len();
        let mut ans = vec![vec!['\0'; m]; n];

        for (i, row) in box_grid.iter().enumerate() {
            let mut cnt = 0;
            for (j, &ch) in row.into_iter().enumerate() {
                let mut ch = ch;
                if ch == '#' {
                    // 石头
                    cnt += 1;
                    ch = '.'; // 先把石头清空
                }
                ans[j][m - 1 - i] = ch;
                if j == n - 1 || row[j + 1] == '*' {
                    // 下一个格子是障碍物
                    // 石头垂直掉落后，从 j 往前 cnt 个格子都是石头
                    for k in j - cnt + 1..=j {
                        ans[k][m - 1 - i] = '#';
                    }
                    cnt = 0; // 重置计数器
                }
            }
        }

        ans
    }
}

fn main() {
    let tests = vec![
        (
            vec![vec!['#', '.', '#']],
            vec![vec!['.'], vec!['#'], vec!['#']],
        ),
        (
            vec![vec!['#', '.', '*', '.'], vec!['#', '#', '*', '.']],
            vec![
                vec!['#', '.'],
                vec!['#', '#'],
                vec!['*', '*'],
                vec!['.', '.'],
            ],
        ),
        (
            vec![
                vec!['#', '#', '*', '.', '*', '.'],
                vec!['#', '#', '#', '*', '.', '.'],
                vec!['#', '#', '#', '.', '#', '.'],
            ],
            vec![
                vec!['.', '#', '#'],
                vec!['.', '#', '#'],
                vec!['#', '#', '*'],
                vec!['#', '*', '.'],
                vec!['#', '.', '*'],
                vec!['#', '.', '.'],
            ],
        ),
    ];

    for (box_grid, expected) in tests {
        assert_eq!(Solution::rotate_the_box(box_grid), expected);
    }
}
