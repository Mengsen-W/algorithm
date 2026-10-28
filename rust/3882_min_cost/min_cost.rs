struct Solution;

impl Solution {
    pub fn min_cost(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();

        let mut dp = vec![[false; 1024]; n];
        let mut next = [false; 1024];

        dp[0][grid[0][0] as usize] = true;

        for i in 0..m {
            for j in 0..n {
                if i == 0 && j == 0 {
                    continue;
                }

                let value = grid[i][j] as usize;
                next.fill(false);

                for k in 0..1024 {
                    if dp[j][k] || (j > 0 && dp[j - 1][k]) {
                        next[k ^ value] = true;
                    }
                }

                dp[j] = next;
            }
        }

        dp[n - 1]
            .iter()
            .position(|&value| value)
            .map_or(-1, |index| index as i32)
    }
}

fn main() {
    let tests = vec![
        (vec![vec![1, 2], vec![3, 4]], 6),
        (vec![vec![6, 7], vec![5, 8]], 9),
        (vec![vec![2, 7, 5]], 0),
    ];

    for (grid, expected) in tests {
        assert_eq!(Solution::min_cost(grid), expected);
    }
}
