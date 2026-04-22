struct Solution;

impl Solution {
    pub fn row_and_maximum_ones(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let mut max_ones = 0;
        let mut row_index = 0;
        for (i, row) in mat.iter().enumerate() {
            let tot = row.iter().sum();
            if tot > max_ones {
                max_ones = tot;
                row_index = i;
            }
        }
        vec![row_index as i32, max_ones]
    }
}

fn main() {
    let tests = vec![
        (vec![vec![0, 1], vec![1, 0]], vec![0, 1]),
        (vec![vec![0, 0, 0], vec![0, 1, 1]], vec![1, 2]),
        (vec![vec![0, 0], vec![1, 1], vec![0, 0]], vec![1, 2]),
    ];

    for (mat, ans) in tests {
        assert_eq!(Solution::row_and_maximum_ones(mat), ans);
    }
}
