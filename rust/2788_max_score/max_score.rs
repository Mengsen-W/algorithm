struct Solution;
impl Solution {
    pub fn max_score(nums: Vec<i32>, x: i32) -> i64 {
        let mut res = nums[0] as i64;
        let mut dp: Vec<i64> = vec![i32::MIN as i64, i32::MIN as i64];
        dp[(nums[0] % 2) as usize] = nums[0] as i64;
        for i in 1..nums.len() {
            let parity = (nums[i] % 2) as usize;
            let cur = (dp[parity] + nums[i] as i64).max(dp[1 - parity] - x as i64 + nums[i] as i64);
            res = res.max(cur);
            dp[parity] = dp[parity].max(cur);
        }
        return res;
    }
}

fn main() {
    let tests = vec![(vec![2, 3, 6, 1, 9, 2], 5, 13), (vec![2, 4, 6, 8], 3, 20)];

    for (nums, x, ans) in tests {
        assert_eq!(Solution::max_score(nums, x), ans);
    }
}
