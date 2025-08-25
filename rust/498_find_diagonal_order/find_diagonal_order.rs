struct Solution;

impl Solution {
    pub fn find_diagonal_order(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ans = vec![];
        if mat.is_empty() {
            return ans;
        }
        let (mut x, mut y) = (0, 0);

        let mut sum = 0;
        while sum < mat.len() + mat[0].len() - 1 {
            ans.push(mat[x][y]);
            if sum & 1 == 0 {
                if y >= mat[0].len() - 1 {
                    sum += 1;
                    x += 1;
                } else if x <= 0 {
                    sum += 1;
                    y += 1;
                } else {
                    y += 1;
                    x -= 1;
                }
            } else {
                if x >= mat.len() - 1 {
                    y += 1;
                    sum += 1;
                } else if y <= 0 {
                    x += 1;
                    sum += 1;
                } else {
                    x += 1;
                    y -= 1;
                }
            }
        }
        ans
    }
}

fn main() {
    let tests = vec![
        (
            vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
            vec![1, 2, 4, 7, 5, 3, 6, 8, 9],
        ),
        (vec![vec![1, 2], vec![3, 4]], vec![1, 2, 3, 4]),
    ];

    for (mat, ans) in tests {
        assert_eq!(Solution::find_diagonal_order(mat), ans);
    }
}
