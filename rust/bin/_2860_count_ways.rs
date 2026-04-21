struct Solution;

impl Solution {
    pub fn count_ways(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        let n = nums.len();
        let mut res = 0;
        for k in 0..=n {
            // 前 k 个元素的最大值是否小于 k
            if k > 0 && nums[k - 1] >= k as i32 {
                continue;
            }
            // 后 n - k 个元素的最小值是否大于 k
            if k < n && nums[k] <= k as i32 {
                continue;
            }
            res += 1;
        }
        res
    }
}

fn main() {
    let tests = vec![(vec![1, 1], 2), (vec![6, 0, 3, 3, 6, 7, 2, 7], 3)];

    for (nums, ans) in tests {
        assert_eq!(Solution::count_ways(nums), ans);
    }
}
