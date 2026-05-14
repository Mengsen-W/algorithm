struct Solution;

impl Solution {
    pub fn is_good(nums: Vec<i32>) -> bool {
        let n = nums.len() as i32;
        let mut count = vec![0; n as usize];
        for &a in nums.iter() {
            if a < 1 || a >= n {
                return false;
            }
            if a < n - 1 && count[a as usize] > 0 {
                return false;
            }
            if a == n - 1 && count[a as usize] > 1 {
                return false;
            }
            count[a as usize] += 1;
        }
        true
    }
}

fn main() {
    let tests = vec![
        (vec![2, 1, 3], false),
        (vec![1, 3, 3, 2], true),
        (vec![1, 1], true),
        (vec![1, 2, 2, 3, 3, 4], false),
    ];

    for (nums, expected) in tests {
        assert_eq!(Solution::is_good(nums), expected);
    }
}
