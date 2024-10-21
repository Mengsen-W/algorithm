struct Solution;

impl Solution {
    pub fn smallest_range_ii(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort();
        let (mi, ma) = (nums[0], nums[nums.len() - 1]);
        let mut res = ma - mi;
        for i in 0..nums.len() - 1 {
            let (a, b) = (nums[i], nums[i + 1]);
            res = res.min((ma - k).max(a + k) - (mi + k).min(b - k));
        }
        res
    }
}

fn main() {
    let tests = vec![(vec![1], 0, 0), (vec![0, 10], 2, 6), (vec![1, 3, 6], 3, 3)];

    for (nums, k, ans) in tests {
        assert_eq!(Solution::smallest_range_ii(nums, k), ans);
    }
}
