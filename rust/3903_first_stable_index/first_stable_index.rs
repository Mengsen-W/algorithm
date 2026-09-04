struct Solution;

impl Solution {
    pub fn first_stable_index(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        for i in 0..n {
            let mut max_value = nums[i];
            let mut min_value = nums[i];
            for j in 0..i {
                max_value = max_value.max(nums[j]);
            }
            for j in i + 1..n {
                min_value = min_value.min(nums[j]);
            }
            if max_value - min_value <= k {
                return i as i32;
            }
        }
        -1
    }
}

fn main() {
    let tests = vec![
        (vec![5, 0, 1, 4], 3, 3),
        (vec![3, 2, 1], 1, -1),
        (vec![0], 0, 0),
    ];

    for (nums, k, ans) in tests {
        assert_eq!(Solution::first_stable_index(nums, k), ans);
    }
}
