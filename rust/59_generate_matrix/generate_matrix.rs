struct Solution;

impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let mut num = 1;
        let mut matrix = vec![vec![0; n as usize]; n as usize];
        let (mut left, mut right, mut top, mut bottom) = (0, n - 1, 0, n - 1);
        while left <= right && top <= bottom {
            for column in left..=right {
                matrix[top as usize][column as usize] = num;
                num += 1;
            }
            for row in (top + 1)..=bottom {
                matrix[row as usize][right as usize] = num;
                num += 1;
            }
            if left < right && top < bottom {
                for column in ((left + 1)..right).rev() {
                    matrix[bottom as usize][column as usize] = num;
                    num += 1;
                }
                for row in ((top + 1)..=bottom).rev() {
                    matrix[row as usize][left as usize] = num;
                    num += 1;
                }
            }
            left += 1;
            right -= 1;
            top += 1;
            bottom -= 1;
        }
        matrix
    }
}

fn main() {
    let tests = vec![
        (3, vec![vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]]),
        (1, vec![vec![1]]),
    ];

    for (n, ans) in tests {
        assert_eq!(Solution::generate_matrix(n), ans);
    }
}
