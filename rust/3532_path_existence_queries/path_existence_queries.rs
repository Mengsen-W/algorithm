struct Solution;

impl Solution {
    pub fn path_existence_queries(
        n: i32,
        nums: Vec<i32>,
        max_diff: i32,
        queries: Vec<Vec<i32>>,
    ) -> Vec<bool> {
        let mut tags = vec![0; n as usize];
        for i in 1..n as usize {
            tags[i] = tags[i - 1]
                + if nums[i] - nums[i - 1] > max_diff {
                    1
                } else {
                    0
                };
        }

        queries
            .iter()
            .map(|q| tags[q[0] as usize] == tags[q[1] as usize])
            .collect()
    }
}

fn main() {
    let tests = vec![
        (
            4,
            vec![1, 3],
            1,
            vec![vec![0, 0], vec![0, 1]],
            vec![true, false],
        ),
        (
            4,
            vec![2, 5, 6, 8],
            2,
            vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![2, 3]],
            vec![false, false, true, true],
        ),
    ];

    for (n, nums, max_diff, queries, expected) in tests {
        assert!(Solution::path_existence_queries(n, nums, max_diff, queries) == expected);
    }
}
