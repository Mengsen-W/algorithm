struct Solution;

impl Solution {
    pub fn min_sum_of_lengths(arr: Vec<i32>, target: i32) -> i32 {
        let n = arr.len();
        let (mut ans, mut sum, mut left) = ((n + 1) as i32, 0, 0usize);
        let mut dp = vec![n as i32; n + 1];
        for right in 0..n {
            sum += arr[right];
            while sum > target {
                sum -= arr[left];
                left += 1;
            }
            dp[right + 1] = dp[right];
            if sum == target {
                let len = right - left + 1;
                ans = ans.min(len as i32 + dp[left]);
                dp[right + 1] = dp[right].min(len as i32);
            }
        }
        if ans == (n + 1) as i32 {
            -1
        } else {
            ans
        }
    }
}

fn main() {
    let tests = vec![
        (vec![3, 2, 2, 4, 3], 3, 2),
        (vec![7, 3, 4, 7], 7, 2),
        (vec![3, 2, 2, 4, 3], 6, -1),
        (vec![5, 5, 4, 4, 5], 3, -1),
        (vec![3, 1, 1, 1, 5, 1, 2, 1], 3, 3),
    ];

    for (arr, target, ans) in tests {
        assert_eq!(Solution::min_sum_of_lengths(arr, target), ans);
    }
}
