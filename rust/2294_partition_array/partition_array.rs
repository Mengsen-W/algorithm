struct Solution;

impl Solution {
    pub fn partition_array(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();
        let mut ans = 0;
        let mut mn = i32::MIN / 2; // 防止减法溢出
        for x in nums {
            if x - mn > k {
                // 必须分割
                ans += 1;
                mn = x; // mn 是下一段的最小值
            }
        }
        ans
    }
}

fn main() {
    let tests = vec![
        (vec![3, 6, 1, 2, 5], 2, 2),
        (vec![1, 2, 3], 1, 2),
        (vec![2, 2, 4, 5], 0, 3),
    ];

    for (nums, k, expected) in tests {
        assert_eq!(Solution::partition_array(nums, k), expected);
    }
}
