struct Solution;

impl Solution {
    pub fn longest_monotonic_subarray(nums: Vec<i32>) -> i32 {
        let mut ans = 1;
        let mut i = 0;
        let n = nums.len();
        while i < n - 1 {
            if nums[i + 1] == nums[i] {
                i += 1;
                continue;
            }
            let j = i;
            let inc = nums[i + 1] > nums[i];
            i += 2;
            while i < n && nums[i] != nums[i - 1] && (nums[i] > nums[i - 1]) == inc {
                i += 1;
            }
            ans = ans.max(i - j);
            i -= 1;
        }
        ans as i32
    }
}

fn main() {
    let tests = vec![
        (vec![1, 4, 3, 3, 2], 2),
        (vec![3, 3, 3, 3], 1),
        (vec![3, 2, 1], 3),
    ];

    for (nums, ans) in tests {
        assert_eq!(Solution::longest_monotonic_subarray(nums), ans);
    }
}
