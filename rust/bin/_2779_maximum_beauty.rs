struct Solution;
impl Solution {
    pub fn maximum_beauty(nums: Vec<i32>, k: i32) -> i32 {
        let m = nums.iter().max().unwrap();
        let mut diff = vec![0; (m + 2) as usize];
        for x in nums.iter() {
            diff[std::cmp::max(x - k, 0) as usize] += 1;
            diff[std::cmp::min(x + k + 1, m + 1) as usize] -= 1;
        }
        let mut res = 0;
        let mut count = 0;
        for x in diff.iter() {
            count += x;
            res = std::cmp::max(res, count);
        }
        res as i32
    }
}

fn main() {
    let tests = vec![(vec![4, 6, 1, 2], 2, 3), (vec![1, 1, 1, 1], 10, 4)];

    for (nums, k, ans) in tests {
        assert_eq!(Solution::maximum_beauty(nums, k), ans);
    }
}
