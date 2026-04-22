struct Solution;

impl Solution {
    pub fn max_increasing_subarrays(nums: Vec<i32>) -> i32 {
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

        ans
    }
}

fn main() {
    let tests = vec![
        (vec![2, 5, 7, 8, 9, 2, 3, 4, 3, 1], 3),
        (vec![1, 2, 3, 4, 4, 4, 4, 5, 6, 7], 2),
    ];

    for (nums, ans) in tests {
        assert_eq!(Solution::max_increasing_subarrays(nums), ans);
    }
}
