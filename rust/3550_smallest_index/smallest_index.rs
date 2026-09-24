struct Solution;

impl Solution {
    pub fn smallest_index(nums: Vec<i32>) -> i32 {
        for (i, &num) in nums.iter().enumerate() {
            let mut num = num;
            let mut digit_sum = 0;

            while num > 0 {
                digit_sum += num % 10;
                num /= 10;
            }

            if digit_sum == i as i32 {
                return i as i32;
            }
        }

        -1
    }
}

fn main() {
    let tests = vec![
        (vec![1, 3, 2], 2),
        (vec![1, 10, 11], 1),
        (vec![1, 2, 3], -1),
    ];

    for (nums, ans) in tests {
        assert_eq!(Solution::smallest_index(nums), ans);
    }
}
