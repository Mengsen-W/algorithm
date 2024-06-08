struct Solution;

impl Solution {
    pub fn max_operations(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let helper = |i: usize, j: usize, target: i32| -> i32 {
            let mut memo = vec![vec![-1; n]; n];
            fn dfs(i: i32, j: i32, nums: &Vec<i32>, memo: &mut Vec<Vec<i32>>, target: i32) -> i32 {
                if i >= j {
                    return 0;
                }
                if memo[i as usize][j as usize] != -1 {
                    return memo[i as usize][j as usize];
                }
                let mut ans = 0;
                if nums[i as usize] + nums[(i + 1) as usize] == target {
                    ans = ans.max(1 + dfs(i + 2, j, nums, memo, target));
                }
                if nums[(j - 1) as usize] + nums[j as usize] == target {
                    ans = ans.max(1 + dfs(i, j - 2, nums, memo, target));
                }
                if nums[i as usize] + nums[j as usize] == target {
                    ans = ans.max(1 + dfs(i + 1, j - 1, nums, memo, target));
                }
                memo[i as usize][j as usize] = ans;
                ans
            }

            dfs(i as i32, j as i32, &nums, &mut memo, target)
        };

        let mut res = 0;
        res = res.max(helper(0, n - 1, nums[0] + nums[n - 1]));
        res = res.max(helper(0, n - 1, nums[0] + nums[1]));
        res = res.max(helper(0, n - 1, nums[n - 2] + nums[n - 1]));
        res
    }
}

fn main() {
    let tests = vec![(vec![3, 2, 1, 2, 3, 4], 3), (vec![3, 2, 6, 1, 4], 2)];

    for (nums, ans) in tests {
        assert_eq!(Solution::max_operations(nums), ans);
    }
}
