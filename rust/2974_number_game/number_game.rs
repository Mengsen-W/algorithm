struct Solution;

impl Solution {
    pub fn number_game(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort();
        for i in (0..nums.len() - 1).step_by(2) {
            nums.swap(i, i + 1);
        }
        nums
    }
}

fn main() {
    let tests = vec![
        (vec![5, 4, 2, 3], vec![3, 2, 5, 4]),
        (vec![2, 5], vec![5, 2]),
    ];

    for (nums, ans) in tests {
        assert_eq!(Solution::number_game(nums), ans);
    }
}
