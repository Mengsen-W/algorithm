struct Solution;

impl Solution {
    pub fn added_integer(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let max_num1 = *nums1.iter().max().unwrap_or(&i32::MIN);
        let max_num2 = *nums2.iter().max().unwrap_or(&i32::MAX);
        max_num2 - max_num1
    }
}

fn main() {
    let tests = vec![
        (vec![2, 6, 4], vec![9, 7, 5], 3),
        (vec![10], vec![5], -5),
        (vec![1, 1, 1, 1], vec![1, 1, 1, 1], 6),
    ];

    for (nums1, nums2, ans) in tests {
        assert_eq!(Solution::added_integer(nums1, nums2), ans);
    }
}
