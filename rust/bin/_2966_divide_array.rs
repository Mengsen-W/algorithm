struct Solution;

impl Solution {
    pub fn divide_array(mut nums: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        nums.sort();
        let mut res = Vec::new();
        for i in (0..nums.len()).step_by(3) {
            if nums[i + 2] - nums[i] > k {
                return vec![];
            }
            res.push(vec![nums[i], nums[i + 1], nums[i + 2]]);
        }
        res
    }
}

fn main() {
    let tests = vec![
        (
            vec![1, 3, 4, 8, 7, 9, 3, 5, 1],
            2,
            vec![vec![1, 1, 3], vec![3, 4, 5], vec![7, 8, 9]],
        ),
        (vec![2, 4, 2, 2, 5, 2], 2, vec![]),
        (
            vec![4, 2, 9, 8, 2, 12, 7, 12, 10, 5, 8, 5, 5, 7, 9, 2, 5, 11],
            14,
            vec![
                vec![2, 2, 12],
                vec![4, 5, 5],
                vec![5, 9, 7],
                vec![7, 8, 5],
                vec![5, 9, 10],
                vec![11, 12, 2],
            ],
        ),
    ];

    for (nums, k, expected) in tests {
        assert_eq!(Solution::divide_array(nums, k), expected);
    }
}
