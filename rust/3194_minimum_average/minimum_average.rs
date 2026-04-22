struct Solution;

impl Solution {
    pub fn minimum_average(nums: Vec<i32>) -> f64 {
        let mut nums = nums;
        nums.sort();
        let n = nums.len();
        let mut res = f64::INFINITY;
        for i in 0..n / 2 {
            let average = (nums[i] as f64 + nums[n - 1 - i] as f64) / 2.0;
            res = res.min(average);
        }
        res
    }
}

fn main() {
    let tests = vec![
        (vec![7, 8, 3, 4, 15, 13, 4, 1], 5.5),
        (vec![1, 9, 8, 3, 10, 5], 5.5),
        (vec![1, 2, 3, 7, 8, 9], 5.0),
    ];

    for (nums, ans) in tests {
        assert_eq!(Solution::minimum_average(nums), ans);
    }
}
