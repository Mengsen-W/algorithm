struct Solution;

impl Solution {
    pub fn count_hill_valley(nums: Vec<i32>) -> i32 {
        let mut res = 0; // 峰与谷的数量
        let n = nums.len();
        for i in 1..n - 1 {
            if nums[i] == nums[i - 1] {
                // 去重
                continue;
            }
            let mut left = 0; // 左边可能的不相等邻居对应状态
            for j in (0..i).rev() {
                if nums[j] > nums[i] {
                    left = 1;
                    break;
                } else if nums[j] < nums[i] {
                    left = -1;
                    break;
                }
            }
            let mut right = 0; // 右边可能的不相等邻居对应状态
            for j in i + 1..n {
                if nums[j] > nums[i] {
                    right = 1;
                    break;
                } else if nums[j] < nums[i] {
                    right = -1;
                    break;
                }
            }
            if left == right && left != 0 {
                // 此时下标 i 为峰或谷的一部分
                res += 1;
            }
        }
        res
    }
}

fn main() {
    let tests = vec![(vec![2, 4, 1, 1, 6, 5], 3), (vec![6, 6, 5, 5, 4, 1], 0)];

    for (nums, ans) in tests {
        assert_eq!(Solution::count_hill_valley(nums), ans);
    }
}
