struct Solution;

impl Solution {
    pub fn count_alternating_subarrays(nums: Vec<i32>) -> i64 {
        let mut res: i64 = 0;
        let mut cur = 0;
        let mut pre = -1;
        for &a in &nums {
            if pre != a {
                cur += 1;
            } else {
                cur = 1;
            }
            pre = a;
            res += cur;
        }
        res
    }
}

fn main() {
    let tests = vec![(vec![0, 1, 1, 1], 5), (vec![1, 0, 1, 0], 10)];

    for (nums, ans) in tests {
        assert_eq!(Solution::count_alternating_subarrays(nums), ans);
    }
}
