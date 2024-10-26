struct Solution;

impl Solution {
    pub fn max_total_reward(reward_values: Vec<i32>) -> i32 {
        let mut reward_values = reward_values;
        reward_values.sort_unstable();
        reward_values.dedup();
        let max = reward_values.iter().fold(0, |acc, &x| acc.max(x));
        let mut dp = vec![false; 2 * max as usize + 1];
        dp[0] = true;
        unsafe {
            for v in reward_values.into_iter() {
                for x in (v..v << 1).rev() {
                    *(dp.get_unchecked_mut(x as usize)) |= *dp.get_unchecked((x - v) as usize);
                }
            }
        }
        dp.iter().enumerate().rfind(|(_, &x)| x).unwrap().0 as i32
    }
}

fn main() {
    let tests = vec![(vec![1, 1, 3, 3], 4), (vec![1, 6, 4, 3, 2], 11)];

    for (reward_values, ans) in tests {
        assert_eq!(Solution::max_total_reward(reward_values), ans);
    }
}
