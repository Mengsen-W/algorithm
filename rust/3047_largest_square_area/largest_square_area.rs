struct Solution;

impl Solution {
    pub fn largest_square_area(bottom_left: Vec<Vec<i32>>, top_right: Vec<Vec<i32>>) -> i64 {
        use std::cmp::{max, min};
        let n = bottom_left.len();
        let mut max_side = 0;

        for i in 0..n {
            for j in i + 1..n {
                let w = min(top_right[i][0], top_right[j][0])
                    - max(bottom_left[i][0], bottom_left[j][0]);
                let h = min(top_right[i][1], top_right[j][1])
                    - max(bottom_left[i][1], bottom_left[j][1]);

                max_side = max(max_side, min(w, h));
            }
        }

        (max_side as i64) * (max_side as i64)
    }
}

fn main() {
    let tests = vec![
        (
            vec![vec![1, 1], vec![2, 2], vec![3, 1]],
            vec![vec![3, 3], vec![4, 4], vec![6, 6]],
            1,
        ),
        (
            vec![vec![1, 1], vec![2, 2], vec![1, 2]],
            vec![vec![3, 3], vec![4, 4], vec![3, 4]],
            1,
        ),
        (
            vec![vec![1, 1], vec![3, 3], vec![3, 1]],
            vec![vec![2, 2], vec![4, 4], vec![4, 2]],
            0,
        ),
    ];

    for (bottom_left, top_right, expected) in tests {
        assert_eq!(
            Solution::largest_square_area(bottom_left, top_right),
            expected
        );
    }
}
