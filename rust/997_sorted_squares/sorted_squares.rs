struct Solution;

impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut ans: Vec<i32> = nums.iter().map(|&num| num * num).collect();
        ans.sort();
        ans
    }
}

fn main() {
    let tests = vec![
        (vec![-4, -1, 0, 3, 10], vec![0, 1, 9, 16, 100]),
        (vec![-7, -3, 2, 3, 11], vec![4, 9, 9, 49, 121]),
    ];

    for (nums, ans) in tests {
        assert_eq!(Solution::sorted_squares(nums), ans);
    }
}
