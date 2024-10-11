struct Solution;

impl Solution {
    pub fn number_of_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i32 {
        use std::collections::HashMap;
        let mut count: HashMap<i32, i32> = HashMap::new();
        let mut count2: HashMap<i32, i32> = HashMap::new();
        let mut res = 0;
        let mut max1 = 0;
        for &num in &nums1 {
            *count.entry(num).or_insert(0) += 1;
            max1 = std::cmp::max(max1, num);
        }
        for &num in &nums2 {
            *count2.entry(num).or_insert(0) += 1;
        }
        for (&a, &cnt) in &count2 {
            for b in (a * k..=max1).step_by((a * k) as usize) {
                if let Some(&value) = count.get(&b) {
                    res += value * cnt;
                }
            }
        }
        res
    }
}

fn main() {
    let tests = vec![
        (vec![1, 3, 4], vec![1, 3, 4], 1, 5),
        (vec![1, 2, 4, 12], vec![2, 4], 3, 2),
    ];

    for (nums1, nums2, k, ans) in tests {
        assert_eq!(Solution::number_of_pairs(nums1, nums2, k), ans);
    }
}
