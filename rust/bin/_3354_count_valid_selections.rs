struct Solution;

impl Solution {
    pub fn count_valid_selections(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let sum: i32 = nums.iter().sum();
        let mut left = 0;
        let mut right = sum;
        for &x in &nums {
            if x == 0 {
                if left - right >= 0 && left - right <= 1 {
                    ans += 1;
                }
                if right - left >= 0 && right - left <= 1 {
                    ans += 1;
                }
            } else {
                left += x;
                right -= x;
            }
        }
        ans
    }
}

fn main() {
    let tests = vec![(vec![1, 0, 2, 0, 3], 2), (vec![2, 3, 4, 0, 4, 1, 0], 0)];

    for (nums, ans) in tests {
        assert_eq!(Solution::count_valid_selections(nums), ans);
    }
}
