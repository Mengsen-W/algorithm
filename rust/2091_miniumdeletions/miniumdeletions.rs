struct Solution;

impl Solution {
    pub fn minimum_deletions(nums: Vec<i32>) -> i32 {
        let n = nums.len();

        // 找到最小值和最大值的下标
        let mut minidx = 0;
        let mut maxidx = 0;
        for i in 0..n {
            if nums[i] < nums[minidx] {
                minidx = i;
            }
            if nums[i] > nums[maxidx] {
                maxidx = i;
            }
        }
        let l = minidx.min(maxidx) as i32; // 最值下标中的较小值
        let r = minidx.max(maxidx) as i32; // 最值下标中的较大值
        let n = n as i32;

        // 计算三种情况下删除次数的最小值
        (r + 1).min(n - l).min(l + 1 + n - r)
    }
}

fn main() {
    let tests = vec![
        (vec![2, 10, 7, 5, 4, 1, 8, 6], 5),
        (vec![0, -4, 19, 1, 8, -2, -3, 5], 3),
        (vec![101], 1),
    ];

    for (nums, expected) in tests {
        assert_eq!(Solution::minimum_deletions(nums), expected);
    }
}
