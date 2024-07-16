struct Solution;

impl Solution {
    pub fn find_intersection_values(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        use std::collections::HashSet;
        let mut s1 = HashSet::new();
        let mut s2 = HashSet::new();
        for x in nums1.iter() {
            s1.insert(x);
        }
        for x in nums2.iter() {
            s2.insert(x);
        }
        let mut res1 = 0;
        let mut res2 = 0;
        for x in nums1.iter() {
            if s2.contains(x) {
                res1 += 1;
            }
        }
        for x in nums2.iter() {
            if s1.contains(x) {
                res2 += 1;
            }
        }
        [res1, res2].to_vec()
    }
}

fn main() {
    let tests = vec![
        (vec![2, 3, 2], vec![1, 2], vec![2, 1]),
        (vec![4, 3, 2, 3, 1], vec![2, 2, 5, 2, 3, 6], vec![3, 4]),
        (vec![3, 4, 2, 3], vec![1, 5], vec![0, 0]),
    ];

    for (nums1, nums2, ans) in tests {
        assert_eq!(Solution::find_intersection_values(nums1, nums2), ans);
    }
}
