struct Solution;

use std::collections::HashMap;
const INF: i32 = (1 << 20) - 1;

impl Solution {
    pub fn minimum_value_sum(nums: Vec<i32>, and_values: Vec<i32>) -> i32 {
        let n = nums.len();
        let m = and_values.len();
        let mut memo: Vec<HashMap<i32, i32>> = vec![HashMap::new(); n * m];
        let res = Self::dfs(0, 0, INF, &nums, &and_values, &mut memo);
        if res < INF {
            res
        } else {
            -1
        }
    }

    fn dfs(
        i: usize,
        j: usize,
        cur: i32,
        nums: &[i32],
        and_values: &[i32],
        memo: &mut Vec<HashMap<i32, i32>>,
    ) -> i32 {
        let n = nums.len();
        let m = and_values.len();
        let key = i * m + j;
        if i == n && j == m {
            return 0;
        }
        if i == n || j == m {
            return INF;
        }

        if let Some(&value) = memo[key].get(&cur) {
            return value;
        }

        let new_cur = cur & nums[i];
        if (new_cur & and_values[j]) < and_values[j] {
            return INF;
        }
        let mut res = Self::dfs(i + 1, j, new_cur, nums, and_values, memo);
        if new_cur == and_values[j] {
            res = res.min(Self::dfs(i + 1, j + 1, INF, nums, and_values, memo) + nums[i]);
        }

        memo[key].insert(cur, res);
        res
    }
}

fn main() {
    let tests = vec![
        (vec![1, 4, 3, 3, 2], vec![0, 3, 3, 2], 12),
        (vec![2, 3, 5, 7, 7, 7, 5], vec![0, 7, 5], 17),
        (vec![1, 2, 3, 4], vec![2], -1),
    ];

    for (nums, and_values, ans) in tests {
        assert_eq!(Solution::minimum_value_sum(nums, and_values), ans);
    }
}
