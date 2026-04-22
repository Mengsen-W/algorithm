struct Solution;

impl Solution {
    pub fn count_complete_subarrays(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        let mut res = 0;
        let mut cnt = HashMap::new();
        let n = nums.len();
        let mut right = 0;
        let distinct = nums.iter().collect::<std::collections::HashSet<_>>().len();

        for left in 0..n {
            if left > 0 {
                let remove = nums[left - 1];
                *cnt.get_mut(&remove).unwrap() -= 1;
                if cnt[&remove] == 0 {
                    cnt.remove(&remove);
                }
            }
            while right < n && cnt.len() < distinct {
                let add = nums[right];
                *cnt.entry(add).or_insert(0) += 1;
                right += 1;
            }
            if cnt.len() == distinct {
                res += (n - right + 1) as i32;
            }
        }
        res
    }
}

fn main() {
    let tests = vec![(vec![1, 3, 1, 2, 2], 4), (vec![5, 5, 5, 5], 10)];

    for (nums, ans) in tests {
        assert_eq!(Solution::count_complete_subarrays(nums), ans);
    }
}
