struct Solution;

impl Solution {
    pub fn can_alice_win(nums: Vec<i32>) -> bool {
        let mut single_digit_sum: i32 = 0;
        let mut double_digit_sum: i32 = 0;
        for &num in &nums {
            if num < 10 {
                single_digit_sum += num;
            } else {
                double_digit_sum += num;
            }
        }
        (single_digit_sum != double_digit_sum) as bool
    }
}

fn main() {
    let tests = vec![
        (vec![1, 2, 3, 4, 10], false),
        (vec![1, 2, 3, 4, 5, 14], true),
        (vec![5, 5, 5, 25], true),
    ];

    for (nums, ans) in tests {
        assert_eq!(Solution::can_alice_win(nums), ans);
    }
}
