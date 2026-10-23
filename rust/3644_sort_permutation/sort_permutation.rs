struct Solution;

impl Solution {
    pub fn sort_permutation(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut res = i32::MAX;
        for i in 0..n {
            if nums[i] == i as i32 {
                continue;
            }
            res &= nums[i];
        }
        if res == i32::MAX {
            0
        } else {
            res
        }
    }
}

fn main() {
    let tests = vec![
        (vec![0, 3, 2, 1], 1),
        (vec![0, 1, 3, 2], 2),
        (vec![3, 2, 1, 0], 0),
    ];

    for (nums, ans) in tests {
        assert_eq!(Solution::sort_permutation(nums), ans);
    }
}
