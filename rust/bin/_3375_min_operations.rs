struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        use std::collections::HashSet;
        let mut st = HashSet::new();
        for x in nums {
            if x < k {
                return -1;
            } else if x > k {
                st.insert(x);
            }
        }
        st.len() as i32
    }
}

fn main() {
    let tests = vec![
        (vec![5, 2, 5, 4, 5], 2, 2),
        (vec![2, 1, 2], 2, -1),
        (vec![9, 7, 5, 3], 1, 4),
    ];

    for (nums, k, ans) in tests {
        assert_eq!(Solution::min_operations(nums, k), ans);
    }
}
