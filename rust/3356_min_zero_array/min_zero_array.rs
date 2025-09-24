struct Solution;

impl Solution {
    pub fn min_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        let mut left = 0;
        let mut right = queries.len();
        if !Self::check(&nums, &queries, right) {
            return -1;
        }
        while left < right {
            let k = (left + right) / 2;
            if Self::check(&nums, &queries, k) {
                right = k;
            } else {
                left = k + 1;
            }
        }
        left as i32
    }

    fn check(nums: &[i32], queries: &[Vec<i32>], k: usize) -> bool {
        let mut delta_array = vec![0; nums.len() + 1];
        for i in 0..k {
            let left = queries[i][0] as usize;
            let right = queries[i][1] as usize;
            let value = queries[i][2];
            delta_array[left] += value;
            delta_array[right + 1] -= value;
        }
        let mut operation_counts = Vec::with_capacity(delta_array.len());
        let mut current_operations = 0;
        for &delta in &delta_array {
            current_operations += delta;
            operation_counts.push(current_operations);
        }
        for i in 0..nums.len() {
            if operation_counts[i] < nums[i] {
                return false;
            }
        }
        true
    }
}

fn main() {
    let tests = vec![
        (
            vec![2, 0, 2],
            vec![vec![0, 2, 1], vec![0, 2, 1], vec![1, 1, 3]],
            2,
        ),
        (vec![4, 3, 2, 1], vec![vec![1, 3, 2], vec![0, 2, 1]], -1),
    ];

    for (nums, queries, expected) in tests {
        assert_eq!(Solution::min_zero_array(nums, queries), expected);
    }
}
