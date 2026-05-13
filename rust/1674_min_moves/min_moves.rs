struct Solution;

impl Solution {
    pub fn min_moves(nums: Vec<i32>, limit: i32) -> i32 {
        use std::collections::HashMap;
        let n = nums.len();
        let mut sum_count = HashMap::new();
        let mut min_arr = Vec::with_capacity(n / 2);
        let mut max_arr = Vec::with_capacity(n / 2);

        for i in 0..n / 2 {
            let a = nums[i].min(nums[n - 1 - i]);
            let b = nums[i].max(nums[n - 1 - i]);

            *sum_count.entry(a + b).or_insert(0) += 1;
            min_arr.push(a);
            max_arr.push(b);
        }

        min_arr.sort_unstable();
        max_arr.sort_unstable();

        let mut min_ops = n;

        for c in 2..=2 * limit {
            let add_left = n / 2 - min_arr.partition_point(|&x| x < c);
            let add_right = max_arr.partition_point(|&x| x < c - limit);

            let current_ops = n / 2 + add_left + add_right - sum_count.get(&c).unwrap_or(&0);
            min_ops = min_ops.min(current_ops);
        }

        min_ops as i32
    }
}

fn main() {
    let tests = vec![
        (vec![1, 2, 4, 3], 4, 1),
        (vec![1, 2, 2, 1], 2, 2),
        (vec![1, 2, 1, 2], 2, 0),
    ];

    for (nums, limit, expected) in tests {
        assert_eq!(Solution::min_moves(nums, limit), expected);
    }
}
