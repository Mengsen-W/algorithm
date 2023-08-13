/*
 * @Date: 2023-08-13
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-08-13
 * @FilePath: /algorithm/rust/88_merge/merge.rs
 */

struct Solution;
impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut i1 = m as usize;
        let mut i2 = n as usize;
        let mut i = (m + n) as usize;

        // nums2 处理完就可以结束了
        while i2 > 0 {
            if i1 == 0 || nums2[i2 - 1] > nums1[i1 - 1] {
                nums1[i - 1] = nums2[i2 - 1];
                i2 -= 1;
            } else {
                nums1[i - 1] = nums1[i1 - 1];
                i1 -= 1;
            }
            i -= 1;
        }
    }
}

fn main() {
    let tests = vec![
        (
            vec![1, 2, 3, 0, 0, 0],
            3,
            vec![2, 5, 6],
            3,
            vec![1, 2, 2, 3, 5, 6],
        ),
        (vec![1], 1, vec![], 0, vec![1]),
        (vec![0], 0, vec![1], 1, vec![1]),
    ];

    for (mut nums1, m, mut nums2, n, ans) in tests {
        Solution::merge(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1, ans);
    }
}
