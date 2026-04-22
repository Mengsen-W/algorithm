struct Solution;

impl Solution {
    pub fn num_magic_squares_inside(grid: Vec<Vec<i32>>) -> i32 {
        let rows = grid.len();
        let cols = grid[0].len();
        let mut count = 0;
        if rows < 3 || cols < 3 {
            return 0;
        }
        for r in 0..rows - 2 {
            for c in 0..cols - 2 {
                if grid[r + 1][c + 1] != 5 {
                    continue;
                }
                if Self::is_magic_square(
                    grid[r][c],
                    grid[r][c + 1],
                    grid[r][c + 2],
                    grid[r + 1][c],
                    grid[r + 1][c + 1],
                    grid[r + 1][c + 2],
                    grid[r + 2][c],
                    grid[r + 2][c + 1],
                    grid[r + 2][c + 2],
                ) {
                    count += 1;
                }
            }
        }

        count
    }

    fn is_magic_square(
        a: i32,
        b: i32,
        c: i32,
        d: i32,
        e: i32,
        f: i32,
        g: i32,
        h: i32,
        i: i32,
    ) -> bool {
        let vals = [a, b, c, d, e, f, g, h, i];
        let mut frequency = [0; 16];
        for &value in vals.iter() {
            if value < 1 || value > 9 {
                return false;
            }
            frequency[value as usize] += 1;
        }

        for num in 1..=9 {
            if frequency[num] != 1 {
                return false;
            }
        }

        a + b + c == 15 && // 第一行
        d + e + f == 15 && // 第二行
        g + h + i == 15 && // 第三行
        a + d + g == 15 && // 第一列
        b + e + h == 15 && // 第二列
        c + f + i == 15 && // 第三列
        a + e + i == 15 && // 主对角线
        c + e + g == 15 // 副对角线
    }
}

fn main() {
    let tests = vec![
        (
            vec![vec![4, 3, 8, 4], vec![9, 5, 1, 9], vec![2, 7, 6, 2]],
            1,
        ),
        (vec![vec![8]], 0),
    ];

    for (test, expected) in tests {
        assert_eq!(Solution::num_magic_squares_inside(test), expected);
    }
}
