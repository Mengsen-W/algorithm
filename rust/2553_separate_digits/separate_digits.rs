struct Solution;

impl Solution {
    pub fn separate_digits(nums: Vec<i32>) -> Vec<i32> {
        let mut res = Vec::new();
        for i in (0..nums.len()).rev() {
            let mut x = nums[i];
            while x > 0 {
                res.push(x % 10);
                x /= 10;
            }
        }
        res.reverse();
        res
    }
}

fn main() {
    let tests = vec![
        (vec![13, 25, 83, 77], vec![1, 3, 2, 5, 8, 3, 7, 7]),
        (vec![7, 1, 3, 9], vec![7, 1, 3, 9]),
    ];

    for (nums, expected) in tests {
        assert_eq!(Solution::separate_digits(nums), expected);
    }
}
