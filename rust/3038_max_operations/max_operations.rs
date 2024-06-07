struct Solution;

impl Solution {
    pub fn max_operations(nums: Vec<i32>) -> i32 {
        let mut n = nums.len();
        let mut t = 0;
        let mut i = 1;
        while i < n {
            if nums[i] + nums[i - 1] != nums[1] + nums[0] {
                break;
            }
            t += 1;
            i += 2;
        }
        return t;
    }
}

fn main() {
    let tests = vec![(vec![3, 2, 1, 4, 5], 2), (vec![3, 2, 6, 1, 4], 1)];

    for (nums, ans) in tests {
        assert_eq!(Solution::max_operations(nums), ans);
    }
}
