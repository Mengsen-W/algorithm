struct Solution;

impl Solution {
    pub fn is_array_special(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let n = nums.len();
        let mut dp = vec![1; n];
        for i in 1..n {
            if (nums[i] ^ nums[i - 1]) & 1 != 0 {
                dp[i] = dp[i - 1] + 1;
            }
        }

        let mut res = Vec::with_capacity(queries.len());
        for q in queries {
            let x = q[0] as usize;
            let y = q[1] as usize;
            res.push(dp[y] >= y - x + 1);
        }
        res
    }
}

fn main() {
    let tests = vec![
        (vec![3, 4, 1, 2, 6], vec![vec![0, 4]], vec![false]),
        (
            vec![4, 3, 1, 6],
            vec![vec![0, 2], vec![2, 3]],
            vec![false, true],
        ),
    ];

    for (nums, queries, ans) in tests {
        assert_eq!(Solution::is_array_special(nums, queries), ans);
    }
}
