struct Solution;

impl Solution {
    pub fn has_increasing_subarrays(nums: Vec<i32>, k: i32) -> bool {
        let n = nums.len();
        let mut cnt = 1;
        let mut precnt = 0;
        let mut ans = 0;

        for i in 1..n {
            if nums[i] > nums[i - 1] {
                cnt += 1;
            } else {
                precnt = cnt;
                cnt = 1;
            }
            ans = ans.max(precnt.min(cnt));
            ans = ans.max(cnt / 2);
        }

        ans >= k
    }
}

fn main() {
    let tests = vec![
        (vec![2, 5, 7, 8, 9, 2, 3, 4, 3, 1], 3, true),
        (vec![1, 2, 3, 4, 4, 4, 4, 5, 6, 7], 5, false),
    ];

    for (t, k, ans) in tests {
        assert_eq!(Solution::has_increasing_subarrays(t, k), ans);
    }
}
