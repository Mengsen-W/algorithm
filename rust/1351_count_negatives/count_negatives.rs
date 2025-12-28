struct Solution;

impl Solution {
    pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
        let mut num = 0;
        let m = grid[0].len();
        let mut pos = (grid[0].len() - 1) as i32;

        for row in grid {
            let mut i = pos;
            while i >= 0 {
                if row[i as usize] >= 0 {
                    if i + 1 < m as i32 {
                        pos = i + 1;
                        num += (m as i32) - pos;
                    }
                    break;
                }
                i -= 1;
            }
            if i == -1 {
                num += m as i32;
                pos = -1;
            }
        }

        num
    }
}

fn main() {
    let tests = vec![
        (
            vec![
                vec![4, 3, 2, -1],
                vec![3, 2, 1, -1],
                vec![1, 1, -1, -2],
                vec![-1, -1, -2, -3],
            ],
            8,
        ),
        (vec![vec![3, 2], vec![1, 0]], 0),
    ];

    for (grid, ans) in tests {
        assert_eq!(Solution::count_negatives(grid), ans);
    }
}
