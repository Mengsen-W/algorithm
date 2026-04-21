struct Solution;

impl Solution {
    pub fn num_submat(mat: Vec<Vec<i32>>) -> i32 {
        let n = mat[0].len();
        let mut heights = vec![0; n];
        let mut res = 0;
        for row in mat {
            for i in 0..n {
                heights[i] = if row[i] == 0 { 0 } else { heights[i] + 1 };
            }
            let mut stack: Vec<(i32, i32, i32)> = vec![(-1, 0, -1)];
            for (i, &h) in heights.iter().enumerate() {
                let i = i as i32;
                let h = h as i32;
                while stack.last().unwrap().2 >= h {
                    stack.pop();
                }
                let (j, prev) = (stack.last().unwrap().0, stack.last().unwrap().1);
                let cur = prev + (i - j) * h;
                stack.push((i, cur, h));
                res += cur;
            }
        }
        res
    }
}

fn main() {
    let tests = vec![
        (vec![vec![1, 0, 1], vec![1, 1, 0], vec![1, 1, 0]], 13),
        (
            vec![vec![0, 1, 1, 0], vec![0, 1, 1, 1], vec![1, 1, 1, 0]],
            24,
        ),
    ];

    for (mat, expected) in tests {
        assert_eq!(Solution::num_submat(mat), expected);
    }
}
