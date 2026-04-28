struct Solution;

impl Solution {
    pub fn min_operations(grid: Vec<Vec<i32>>, x: i32) -> i32 {
        let mut nums = Vec::new();
        let m = grid.len();
        let n = grid[0].len();
        let base = grid[0][0];

        for i in 0..m {
            for j in 0..n {
                if (grid[i][j] - base) % x != 0 {
                    return -1;
                }
                nums.push(grid[i][j]);
            }
        }

        nums.sort();
        let choose = nums[nums.len() / 2];
        let mut ans = 0;
        for num in nums {
            ans += (num - choose).abs() / x;
        }
        ans
    }
}

fn main() {
    let tests = vec![
        (vec![vec![2, 4], vec![6, 8]], 2, 4),
        (vec![vec![1, 5], vec![2, 3]], 1, 5),
        (vec![vec![1, 2], vec![3, 4]], 2, -1),
    ];

    for (grid, x, expected) in tests {
        assert_eq!(Solution::min_operations(grid, x), expected);
    }
}
