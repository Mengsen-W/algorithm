struct Solution;

impl Solution {
    pub fn minimize_max(mut nums: Vec<i32>, p: i32) -> i32 {
        nums.sort_unstable();
        let mut left = 0;
        let mut right = nums[nums.len() - 1] - nums[0];

        let check = |mx: i32| -> bool {
            let mut cnt = 0;
            let mut i = 0;
            while i < nums.len() - 1 {
                if nums[i + 1] - nums[i] <= mx {
                    cnt += 1;
                    i += 2;
                } else {
                    i += 1;
                }
            }
            cnt >= p
        };

        while left < right {
            let mid = (left + right) / 2;
            if check(mid) {
                right = mid;
            } else {
                left = mid + 1;
            }
        }

        left
    }
}

fn main() {
    let tests = vec![(vec![10, 1, 2, 7, 1, 3], 2, 1), (vec![4, 2, 1, 2], 1, 0)];

    for (nums, p, ans) in tests {
        assert_eq!(Solution::minimize_max(nums, p), ans);
    }
}
