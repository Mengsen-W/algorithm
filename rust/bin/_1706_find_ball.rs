struct Solution;

impl Solution {
    pub fn find_ball(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let n = grid.first().unwrap_or(&Vec::new()).len() as i32;
        let mut ans = vec![-1; n as usize];
        for j in 0..n {
            let mut col = j; // 初始列
            for row in &grid {
                // 遍历每一行
                let dir = row[col as usize]; // 正负方向
                col += dir; // 更新列
                if col < 0 || col == n || row[col as usize] != dir {
                    // 左边界 || 右边界 || V型
                    col = -1;
                    break;
                }
            }
            if col >= 0 {
                ans[j as usize] = col as i32;
            }
        }
        ans
    }
}

fn main() {
    let tests = vec![
        (
            vec![
                vec![1, 1, 1, -1, -1],
                vec![1, 1, 1, -1, -1],
                vec![-1, -1, -1, 1, 1],
                vec![1, 1, 1, 1, -1],
                vec![-1, -1, -1, -1, -1],
            ],
            vec![1, -1, -1, -1, -1],
        ),
        (vec![vec![-1]], vec![-1]),
        (
            vec![
                vec![1, 1, 1, 1, 1, 1],
                vec![-1, -1, -1, -1, -1, -1],
                vec![1, 1, 1, 1, 1, 1],
                vec![-1, -1, -1, -1, -1, -1],
            ],
            vec![0, 1, 2, 3, 4, -1],
        ),
    ];

    for (grid, ans) in tests {
        assert_eq!(Solution::find_ball(grid), ans);
    }
}
