struct Solution;

impl Solution {
    pub fn find_missing_and_repeated_values(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let n = grid.len();
        let mut count = vec![0; n * n + 1];
        for row in grid.iter() {
            for &val in row.iter() {
                count[val as usize] += 1;
            }
        }

        let mut res = vec![0; 2];
        for i in 1..=n * n {
            if count[i as usize] == 2 {
                res[0] = i as i32;
            }
            if count[i as usize] == 0 {
                res[1] = i as i32;
            }
        }
        res
    }
}

fn main() {
    let tests = vec![
        (vec![vec![1, 3], vec![2, 2]], vec![2, 4]),
        (
            vec![vec![9, 1, 7], vec![8, 9, 2], vec![3, 4, 6]],
            vec![9, 5],
        ),
    ];

    for (grid, ans) in tests {
        assert_eq!(Solution::find_missing_and_repeated_values(grid), ans);
    }
}
