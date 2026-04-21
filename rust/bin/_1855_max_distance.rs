struct Solution;

impl Solution {
    pub fn max_distance(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let n1 = nums1.len();
        let n2 = nums2.len();
        let mut i = 0;
        let mut res = 0;

        for j in 0..n2 {
            while i < n1 && nums1[i] > nums2[j] {
                i += 1;
            }
            if i < n1 {
                res = res.max((j as i32) - (i as i32));
            }
        }

        res
    }
}

fn main() {
    let tests = vec![
        (vec![55, 30, 5, 4, 2], vec![100, 20, 10, 10, 5], 2),
        (vec![2, 2, 2], vec![10, 10, 1], 1),
        (vec![30, 29, 19, 5], vec![25, 25, 25, 25, 25], 2),
    ];

    for (nums1, nums2, expected) in tests {
        assert_eq!(Solution::max_distance(nums1, nums2), expected);
    }
}
