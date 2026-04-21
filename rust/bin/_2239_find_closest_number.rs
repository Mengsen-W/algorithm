struct Solution;

impl Solution {
    pub fn find_closest_number(nums: Vec<i32>) -> i32 {
        let mut res = nums[0]; // 已遍历元素中绝对值最小且数值最大的元素
        let mut dis = nums[0].abs(); // 已遍历元素的最小绝对值
        for &num in nums.iter() {
            if num.abs() < dis {
                dis = num.abs();
                res = num;
            } else if num.abs() == dis {
                res = res.max(num);
            }
        }
        res
    }
}

fn main() {
    let tests = vec![(vec![-4, -2, 1, 4, 8], 1), (vec![2, -1, 1], 1)];

    for (nums, ans) in tests {
        assert_eq!(Solution::find_closest_number(nums), ans);
    }
}
