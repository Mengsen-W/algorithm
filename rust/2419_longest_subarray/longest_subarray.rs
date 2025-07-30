struct Solution;

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let mut max_value = nums[0];
        let mut ans = 1;
        let mut cnt = 1;
        for i in 1..nums.len() {
            if nums[i] > max_value {
                ans = 1;
                cnt = 1;
                max_value = nums[i];
            } else if nums[i] < max_value {
                cnt = 0;
            } else if nums[i] == max_value {
                cnt += 1;
            }
            ans = ans.max(cnt);
        }
        ans
    }
}

fn main() {
    let tests = vec![(vec![1, 2, 3, 3, 2, 2], 2), (vec![1, 2, 3, 4], 1)];

    for (nums, expected) in tests {
        assert_eq!(Solution::longest_subarray(nums), expected);
    }
}
