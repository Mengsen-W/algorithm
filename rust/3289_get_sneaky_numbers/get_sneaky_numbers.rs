struct Solution;

impl Solution {
    pub fn get_sneaky_numbers(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len() as i32 - 2;
        let sum: f64 = nums.iter().map(|&x| x as f64).sum();
        let squared_sum: f64 = nums.iter().map(|&x| (x * x) as f64).sum();
        let sum2 = sum - (n * (n - 1) / 2) as f64;
        let squared_sum2 = squared_sum - (n * (n - 1) * (2 * n - 1) / 6) as f64;
        let x1 = (sum2 - ((2.0 * squared_sum2 - sum2 * sum2).sqrt())) / 2.0;
        let x2 = (sum2 + ((2.0 * squared_sum2 - sum2 * sum2).sqrt())) / 2.0;
        vec![x1 as i32, x2 as i32]
    }
}

fn main() {
    let tests = vec![
        (vec![0, 1, 1, 0], vec![0, 1]),
        (vec![0, 3, 2, 1, 3, 2], vec![2, 3]),
        (vec![7, 1, 5, 4, 3, 4, 6, 0, 9, 5, 8, 2], vec![4, 5]),
    ];

    for (nums, ans) in tests {
        assert_eq!(Solution::get_sneaky_numbers(nums), ans);
    }
}
