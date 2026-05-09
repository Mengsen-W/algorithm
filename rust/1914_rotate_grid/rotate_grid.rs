struct Solution;

impl Solution {
    pub fn rotate_grid(mut grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let m = grid.len();
        let n = grid[0].len();
        let nlayer = (m / 2).min(n / 2); // 层数
        let k = k as usize;
        // 从左上角起逆时针枚举每一层
        for layer in 0..nlayer {
            let mut r = Vec::new();
            let mut c = Vec::new();
            let mut val = Vec::new(); // 每个元素的行下标，列下标与数值
            for i in layer..m - layer - 1 {
                // 左
                r.push(i);
                c.push(layer);
                val.push(grid[i][layer]);
            }
            for j in layer..n - layer - 1 {
                // 下
                r.push(m - layer - 1);
                c.push(j);
                val.push(grid[m - layer - 1][j]);
            }
            for i in (layer + 1..=m - layer - 1).rev() {
                // 右
                r.push(i);
                c.push(n - layer - 1);
                val.push(grid[i][n - layer - 1]);
            }
            for j in (layer + 1..=n - layer - 1).rev() {
                // 上
                r.push(layer);
                c.push(j);
                val.push(grid[layer][j]);
            }
            let total = val.len(); // 每一层的元素总数
            let kk = k % total; // 等效轮转次数 找到每个下标对应的轮转后的取值
            for i in 0..total {
                let idx = (i + total - kk) % total; // 轮转后取值对应的下标
                grid[r[i]][c[i]] = val[idx];
            }
        }
        grid
    }
}

fn main() {
    let tests = vec![
        (
            vec![vec![40, 10], vec![30, 20]],
            1,
            vec![vec![10, 20], vec![40, 30]],
        ),
        (
            vec![
                vec![1, 2, 3, 4],
                vec![5, 6, 7, 8],
                vec![9, 10, 11, 12],
                vec![13, 14, 15, 16],
            ],
            2,
            vec![
                vec![3, 4, 8, 12],
                vec![2, 11, 10, 16],
                vec![1, 7, 6, 15],
                vec![5, 9, 13, 14],
            ],
        ),
    ];

    for (grid, k, expected) in tests {
        assert_eq!(Solution::rotate_grid(grid, k), expected);
    }
}
