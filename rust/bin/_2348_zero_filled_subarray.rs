struct Solution;

impl Solution {
    pub fn zero_filled_subarray(nums: Vec<i32>) -> i64 {
        let mut ans = 0;
        let mut cnt0 = 0;
        for x in nums {
            if x != 0 {
                cnt0 = 0;
            } else {
                cnt0 += 1; // 右端点为 i 的全 0 子数组比右端点为 i-1 的全 0 子数组多一个
                ans += cnt0 as i64;
            }
        }
        ans
    }
}

fn main() {
    let tests = vec![
        (vec![1, 3, 0, 0, 2, 0, 0, 4], 6),
        (vec![0, 0, 0, 2, 0, 0], 9),
        (vec![2, 10, 2019], 0),
    ];

    for (nums, expected) in tests {
        assert_eq!(Solution::zero_filled_subarray(nums), expected);
    }
}
