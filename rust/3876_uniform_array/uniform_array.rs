struct Solution;

impl Solution {
    pub fn uniform_array(nums1: Vec<i32>) -> bool {
        let mut mn = nums1[0];
        let mut has_odd = false;
        for &v in &nums1 {
            if v < mn {
                mn = v;
            }
            if (v & 1) == 1 {
                has_odd = true;
            }
        }
        if (mn & 1) == 1 {
            return true;
        }
        return !has_odd;
    }
}

fn main() {
    let tests = vec![
        (vec![1, 4, 7], true),
        (vec![2, 3], false),
        (vec![4, 6], true),
    ];

    for (nums1, ans) in tests {
        assert_eq!(Solution::uniform_array(nums1), ans);
    }
}
