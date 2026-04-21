struct Solution;

impl Solution {
    pub fn duplicate_numbers_xor(nums: Vec<i32>) -> i32 {
        use std::collections::HashSet;
        let mut cnt = HashSet::new();
        let mut res = 0;

        for num in nums {
            if cnt.contains(&num) {
                res ^= num;
            } else {
                cnt.insert(num);
            }
        }
        res
    }
}

fn main() {
    let tests = vec![
        (vec![1, 2, 1, 3], 1),
        (vec![1, 2, 3], 0),
        (vec![1, 2, 2, 1], 3),
    ];

    for (nums, ans) in tests {
        assert_eq!(Solution::duplicate_numbers_xor(nums), ans);
    }
}
