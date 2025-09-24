struct Solution;

impl Solution {
    pub fn sort_matrix(mut grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = grid.len();

        for i in 0..n {
            let mut tmp: Vec<i32> = (0..(n - i)).map(|j| grid[i + j][j]).collect();
            tmp.sort_by(|a, b| b.cmp(a));
            for j in 0..(n - i) {
                grid[i + j][j] = tmp[j];
            }
        }

        for j in 1..n {
            let mut tmp: Vec<i32> = (0..(n - j)).map(|i| grid[i][j + i]).collect();
            tmp.sort();
            for i in 0..(n - j) {
                grid[i][j + i] = tmp[i];
            }
        }

        grid
    }
}

fn main() {
    let tests = vec![
        (
            vec![vec![1, 7, 3], vec![9, 8, 2], vec![4, 5, 6]],
            vec![vec![8, 2, 3], vec![9, 6, 7], vec![4, 5, 1]],
        ),
        (vec![vec![0, 1], vec![1, 2]], vec![vec![2, 1], vec![1, 0]]),
        (vec![vec![1]], vec![vec![1]]),
    ];

    for (input, expected) in tests {
        assert_eq!(Solution::sort_matrix(input), expected);
    }
}
