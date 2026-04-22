struct Solution;

impl Solution {
    pub fn results_array(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let n = nums.len();
        let mut cnt = 0;
        let mut ans = vec![-1; n - k as usize + 1];

        for i in 0..n {
            cnt = if i == 0 || nums[i] - nums[i - 1] != 1 {
                1
            } else {
                cnt + 1
            };
            if cnt >= k {
                ans[i - k as usize + 1] = nums[i];
            }
        }
        ans
    }
}

fn main() {
    let tests = vec![
        (vec![1, 2, 3, 4, 3, 2, 5], 3, vec![3, 4, -1, -1, -1]),
        (vec![2, 2, 2, 2, 2], 4, vec![-1, -1]),
        (vec![3, 2, 3, 2, 3, 2], 2, vec![-1, 3, -1, 3, -1]),
    ];

    for (nums, k, ans) in tests {
        assert_eq!(Solution::results_array(nums, k), ans);
    }
}
