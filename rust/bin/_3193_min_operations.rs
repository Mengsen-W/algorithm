struct Solution;

impl Solution {
    pub fn min_operations(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut ans = 0;
        for i in 0..n {
            if nums[i] == 0 {
                if i > n - 3 {
                    return -1;
                }
                nums[i] ^= 1;
                nums[i + 1] ^= 1;
                nums[i + 2] ^= 1;
                ans += 1;
            }
        }

        ans
    }
}

fn main() {
    let tests = vec![(vec![0, 1, 1, 1, 0, 0], 3), (vec![0, 1, 1, 1], -1)];

    for (nums, ans) in tests {
        assert_eq!(Solution::min_operations(nums), ans);
    }
}
