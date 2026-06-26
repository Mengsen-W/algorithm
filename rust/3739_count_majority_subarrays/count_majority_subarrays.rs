struct Solution;

impl Solution {
    pub fn count_majority_subarrays(nums: Vec<i32>, target: i32) -> i64 {
        let n = nums.len();
        // 表示前缀和为 -n, -(n-1), ..., 0, 1, ..., n 的出现次数，下标偏移 n
        let mut pre = vec![0; n * 2 + 1];
        pre[n] = 1;
        let mut cnt = n as i32;
        let mut ans: i64 = 0;
        let mut presum: i64 = 0;
        for i in 0..n {
            if nums[i] == target {
                presum += pre[cnt as usize] as i64;
                cnt += 1;
                pre[cnt as usize] += 1;
            } else {
                cnt -= 1;
                presum -= pre[cnt as usize] as i64;
                pre[cnt as usize] += 1;
            }
            ans += presum;
        }
        ans
    }
}

fn main() {
    let tests = vec![
        (vec![1, 2, 2, 3], 2, 5),
        (vec![1, 1, 1, 1], 1, 10),
        (vec![1, 2, 3], 4, 0),
    ];

    for (nums, target, expected) in tests {
        assert_eq!(Solution::count_majority_subarrays(nums, target), expected);
    }
}
