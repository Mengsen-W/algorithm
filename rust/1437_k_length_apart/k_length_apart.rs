struct Solution;

impl Solution {
    pub fn k_length_apart(nums: Vec<i32>, k: i32) -> bool {
        let k = k as usize;
        let mut dst = i32::MAX as usize;
        for i in (0..nums.len()).rev() {
            if nums[i] == 1 {
                if dst - i <= k {
                    return false;
                } else {
                    dst = i
                }
            }
        }
        true
    }
}

fn main() {
    let tests = vec![
        (vec![1, 0, 0, 0, 1, 0, 0, 1], 2, true),
        (vec![1, 0, 0, 1, 0, 1], 2, false),
    ];

    for (nums, k, expected) in tests {
        assert_eq!(Solution::k_length_apart(nums, k), expected);
    }
}
