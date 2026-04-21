struct Solution;

impl Solution {
    pub fn ways_to_split_array(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut left: i64 = 0;
        let mut right: i64 = nums.iter().map(|&x| x as i64).sum();
        let mut ans = 0;
        for i in 0..n - 1 {
            left += nums[i] as i64;
            right -= nums[i] as i64;
            if left >= right {
                ans += 1;
            }
        }
        ans
    }
}

fn main() {
    let tests = vec![(vec![10, 4, -8, 7], 2), (vec![2, 3, 1, 0], 2)];

    for (nums, ans) in tests {
        assert_eq!(Solution::ways_to_split_array(nums), ans);
    }
}
