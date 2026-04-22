struct Solution;

impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let s = nums.iter().sum::<i32>();
        let mut left_s = 0;
        for (i, &x) in nums.iter().enumerate() {
            if left_s * 2 == s - x {
                return i as _;
            }
            left_s += x;
        }
        -1
    }
}

fn main() {
    let tests = vec![
        (vec![1, 7, 3, 6, 5, 6], 3),
        (vec![1, 2, 3], -1),
        (vec![2, 1, -1], 0),
    ];

    for (nums, ans) in tests {
        assert_eq!(Solution::pivot_index(nums), ans);
    }
}
