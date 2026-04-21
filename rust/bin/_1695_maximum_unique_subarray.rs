struct Solution;

impl Solution {
    pub fn maximum_unique_subarray(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        let n = nums.len();
        let mut psum = vec![0; n + 1];
        let mut cnt = HashMap::new();
        let mut ans = 0;
        let mut pre = 0;
        for i in 0..n {
            psum[i + 1] = psum[i] + nums[i];
            pre = pre.max(*cnt.get(&nums[i]).unwrap_or(&0));
            ans = ans.max(psum[i + 1] - psum[pre as usize]);
            cnt.insert(nums[i], (i + 1) as i32);
        }
        ans
    }
}

fn main() {
    let tests = vec![
        (vec![4, 2, 4, 5, 6], 17),
        (vec![5, 2, 1, 2, 5, 2, 1, 2, 5], 8),
    ];

    for (nums, expected) in tests {
        assert_eq!(Solution::maximum_unique_subarray(nums), expected);
    }
}
