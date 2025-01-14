struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut res = 0;
        for num in nums {
            if num < k {
                res += 1;
            }
        }
        res
    }
}

fn main() {
    let tests = vec![
        (vec![2, 11, 10, 1, 3], 10, 3),
        (vec![1, 1, 2, 4, 9], 1, 0),
        (vec![1, 1, 2, 4, 9], 9, 4),
    ];

    for (nums, k, ans) in tests {
        assert_eq!(Solution::min_operations(nums, k), ans);
    }
}
