struct Solution;

impl Solution {
    pub fn construct_transformed_array(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        nums.iter()
            .enumerate()
            .map(|(i, &num)| nums[((i as i32 + num) % n as i32 + n as i32) as usize % n])
            .collect()
    }
}

fn main() {
    let tests = vec![
        (vec![3, -2, 1, 1], vec![1, 1, 1, 3]),
        (vec![-1, 4, -1], vec![-1, -1, 4]),
    ];

    for (nums, expected) in tests {
        assert_eq!(Solution::construct_transformed_array(nums), expected);
    }
}
