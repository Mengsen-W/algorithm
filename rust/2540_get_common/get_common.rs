struct Solution;

impl Solution {
    pub fn get_common(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut i: usize = 0;
        let mut j: usize = 0;
        while i < nums1.len() && j < nums2.len() {
            let a = nums1[i];
            let b = nums2[j];
            if a == b {
                return a;
            }
            if a < b {
                i += 1;
            } else {
                j += 1;
            }
        }
        -1
    }
}

fn main() {
    let tests = vec![
        (vec![1, 2, 3], vec![2, 4], 2),
        (vec![1, 2, 3, 6], vec![2, 3, 4, 5], 2),
    ];

    for (nums1, nums2, expected) in tests {
        assert_eq!(Solution::get_common(nums1, nums2), expected);
    }
}
