struct Solution;

impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut nums1 = nums1;
        let mut nums2 = nums2;
        nums1.sort();
        nums2.sort();
        let mut intersection = Vec::new();
        let (mut i, mut j) = (0, 0);
        while i < nums1.len() && j < nums2.len() {
            if nums1[i] < nums2[j] {
                i += 1;
            } else if nums1[i] > nums2[j] {
                j += 1;
            } else {
                intersection.push(nums1[i]);
                i += 1;
                j += 1;
            }
        }
        intersection
    }
}

fn main() {
    let tests = vec![
        (vec![1, 2, 2, 1], vec![2, 2], vec![2, 2]),
        (vec![4, 9, 5], vec![9, 4, 9, 8, 4], vec![4, 9]),
    ];

    for (nums1, nums2, ans) in tests {
        assert_eq!(Solution::intersect(nums1, nums2), ans);
    }
}
