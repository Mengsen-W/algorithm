struct Solution;

impl Solution {
    pub fn path_existence_queries(
        n: i32,
        nums: Vec<i32>,
        max_diff: i32,
        queries: Vec<Vec<i32>>,
    ) -> Vec<i32> {
        let n = n as usize;

        let mut idx: Vec<usize> = (0..n).collect();
        idx.sort_by_key(|&i| nums[i]);

        let mut pos = vec![0; n];
        for (i, &v) in idx.iter().enumerate() {
            pos[v] = i;
        }

        let m = (n as f64).log2().ceil() as usize + 1;
        let mut f = vec![vec![0; m]; n];

        let mut left = 0;
        for i in 0..n {
            while left < i && nums[idx[i]] - nums[idx[left]] > max_diff {
                left += 1;
            }
            f[i][0] = left;
        }

        for j in 1..m {
            for i in 0..n {
                f[i][j] = f[f[i][j - 1]][j - 1];
            }
        }

        let mut res = Vec::new();
        for query in queries {
            let mut x = pos[query[0] as usize];
            let mut y = pos[query[1] as usize];

            if x == y {
                res.push(0);
                continue;
            }

            if x > y {
                std::mem::swap(&mut x, &mut y);
            }

            let mut step = 0;
            for i in (0..m).rev() {
                if f[y][i] > x {
                    y = f[y][i];
                    step += 1 << i;
                }
            }

            if f[y][0] <= x {
                res.push(step + 1);
            } else {
                res.push(-1);
            }
        }

        res
    }
}

fn main() {
    let tests = vec![
        (
            5,
            vec![1, 8, 3, 4, 2],
            3,
            vec![vec![0, 3], vec![2, 4]],
            vec![1, 1],
        ),
        (
            5,
            vec![5, 3, 1, 9, 10],
            2,
            vec![vec![0, 1], vec![0, 2], vec![2, 3], vec![4, 3]],
            vec![1, 2, -1, 1],
        ),
        (
            3,
            vec![3, 6, 1],
            1,
            vec![vec![0, 0], vec![0, 1], vec![1, 2]],
            vec![0, -1, -1],
        ),
    ];

    for (n, pos, m, queries, expected) in tests {
        assert_eq!(
            expected,
            Solution::path_existence_queries(n, pos, m, queries)
        );
    }
}
