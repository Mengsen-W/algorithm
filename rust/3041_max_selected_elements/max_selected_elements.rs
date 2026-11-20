struct Solution;

impl Solution {
    pub fn max_selected_elements(mut nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        nums.sort_unstable();
        let mut dp = HashMap::<i32, i32>::new();
        let mut ans = 0;
        for x in nums {
            let increased = dp
                .get(&(x + 1))
                .copied()
                .unwrap_or(0)
                .max(dp.get(&x).copied().unwrap_or(0) + 1);
            dp.insert(x + 1, increased);
            let unchanged = dp
                .get(&x)
                .copied()
                .unwrap_or(0)
                .max(dp.get(&(x - 1)).copied().unwrap_or(0) + 1);
            dp.insert(x, unchanged);
            ans = ans.max(increased).max(unchanged);
        }
        ans
    }
}

fn main() {
    let tests = vec![(vec![2, 1, 5, 1, 1], 3), (vec![1, 4, 7, 10], 1)];

    for (nums, expected) in tests {
        assert_eq!(Solution::max_selected_elements(nums), expected);
    }
}
