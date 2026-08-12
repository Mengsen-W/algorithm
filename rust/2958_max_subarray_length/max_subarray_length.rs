struct Solution;

impl Solution {
    pub fn max_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
        use std::collections::HashMap;
        let n = nums.len();
        let mut occ = HashMap::new();
        let mut right = -1;
        let mut ans = 0;

        for left in 0..n {
            if left > 0 {
                let key = nums[left - 1];
                if let Some(val) = occ.get_mut(&key) {
                    *val -= 1;
                    if *val == 0 {
                        occ.remove(&key);
                    }
                }
            }

            while right + 1 < n as i32 && *occ.get(&nums[(right + 1) as usize]).unwrap_or(&0) < k {
                right += 1;
                *occ.entry(nums[right as usize]).or_insert(0) += 1;
            }

            ans = ans.max(right - left as i32 + 1);
        }

        ans
    }
}

fn main() {
    let tests = vec![
        (vec![1, 2, 3, 1, 2, 3, 1, 2], 2, 6),
        (vec![1, 2, 1, 2, 1, 2, 1, 2], 1, 2),
        (vec![5, 5, 5, 5, 5, 5, 5], 4, 4),
    ];

    for (nums, k, expected) in tests {
        assert_eq!(Solution::max_subarray_length(nums, k), expected);
    }
}
