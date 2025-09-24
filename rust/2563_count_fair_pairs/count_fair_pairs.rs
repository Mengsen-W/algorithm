struct Solution;

impl Solution {
    pub fn count_fair_pairs(mut nums: Vec<i32>, lower: i32, upper: i32) -> i64 {
        nums.sort();
        let mut ans = 0;
        let (mut left, mut right) = (nums.len(), nums.len());

        for (j, &num) in nums.iter().enumerate() {
            while right > 0 && nums[right - 1] > upper - num {
                right -= 1;
            }
            while left > 0 && nums[left - 1] >= lower - num {
                left -= 1;
            }
            ans += (right.min(j) as i64) - (left.min(j) as i64);
        }

        ans
    }
}

fn main() {
    let tests = vec![
        (vec![0, 1, 7, 4, 4, 5], 3, 6, 6),
        (vec![1, 7, 9, 2, 5], 11, 11, 1),
    ];

    for (nums, lower, upper, ans) in tests {
        assert_eq!(Solution::count_fair_pairs(nums, lower, upper), ans);
    }
}
