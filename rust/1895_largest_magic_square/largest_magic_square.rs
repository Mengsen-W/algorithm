struct Solution;

impl Solution {
    pub fn largest_magic_square(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        // 每一行的前缀和
        let mut rowsum = vec![vec![0; n]; m];
        for i in 0..m {
            rowsum[i][0] = grid[i][0];
            for j in 1..n {
                rowsum[i][j] = rowsum[i][j - 1] + grid[i][j];
            }
        }
        // 每一列的前缀和
        let mut colsum = vec![vec![0; n]; m];
        for j in 0..n {
            colsum[0][j] = grid[0][j];
            for i in 1..m {
                colsum[i][j] = colsum[i - 1][j] + grid[i][j];
            }
        }

        // 从大到小枚举边长 edge
        for edge in (2..=m.min(n)).rev() {
            let edge = edge as i32;
            // 枚举正方形的左上角位置 (i,j)
            for i in 0..=(m as i32 - edge) {
                for j in 0..=(n as i32 - edge) {
                    let i = i as usize;
                    let j = j as usize;
                    // 计算标准值
                    let stdsum =
                        rowsum[i][j + edge as usize - 1] - if j > 0 { rowsum[i][j - 1] } else { 0 };
                    let mut check = true;
                    // 检查每一行
                    for ii in i + 1..i + edge as usize {
                        let sum = rowsum[ii][j + edge as usize - 1]
                            - if j > 0 { rowsum[ii][j - 1] } else { 0 };
                        if sum != stdsum {
                            check = false;
                            break;
                        }
                    }
                    if !check {
                        continue;
                    }
                    // 检查每一列
                    for jj in j..j + edge as usize {
                        let sum = colsum[i + edge as usize - 1][jj]
                            - if i > 0 { colsum[i - 1][jj] } else { 0 };
                        if sum != stdsum {
                            check = false;
                            break;
                        }
                    }
                    if !check {
                        continue;
                    }
                    // 检查对角线
                    let mut d1 = 0;
                    let mut d2 = 0;
                    for k in 0..edge as usize {
                        d1 += grid[i + k][j + k];
                        d2 += grid[i + k][j + edge as usize - 1 - k];
                    }
                    if d1 == stdsum && d2 == stdsum {
                        return edge;
                    }
                }
            }
        }
        1
    }
}

fn main() {
    let tests = vec![
        (
            vec![
                vec![7, 1, 4, 5, 6],
                vec![2, 5, 1, 6, 4],
                vec![1, 5, 4, 3, 2],
                vec![1, 2, 7, 3, 4],
            ],
            3,
        ),
        (
            vec![vec![5, 1, 3, 1], vec![9, 3, 3, 1], vec![1, 3, 3, 8]],
            2,
        ),
    ];

    for (test, ans) in tests {
        assert_eq!(Solution::largest_magic_square(test), ans);
    }
}
