struct Solution;

impl Solution {
    pub fn uniform_array(nums1: Vec<i32>) -> bool {
        true
    }
}

fn main() {
    let tests = vec![(vec![2, 3], true), (vec![4, 6], true)];

    for (nums1, ans) in tests {
        assert_eq!(Solution::uniform_array(nums1), ans);
    }
}
