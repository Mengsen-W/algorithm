/*
 * @Date: 2022-12-29
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-12-29
 * @FilePath: /algorithm/2023_two_out_of_three/two_out_of_three.rs
 */

pub fn two_out_of_three(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>) -> Vec<i32> {
    let mut map = vec![0; 101];
    nums1.iter().for_each(|k| map[(*k) as usize] |= 1 << 0);
    nums2.iter().for_each(|k| map[(*k) as usize] |= 1 << 1);
    nums3.iter().for_each(|k| map[(*k) as usize] |= 1 << 2);
    map.iter().enumerate().fold(vec![], |mut ans, (idx, &x)| {
        if x & (x - 1) != 0 {
            ans.push(idx as i32);
        }
        ans
    })
}

fn main() {
    {
        let nums1 = vec![1, 1, 3, 2];
        let nums2 = vec![2, 3];
        let nums3 = vec![3];
        let ans = vec![2, 3];
        assert_eq!(two_out_of_three(nums1, nums2, nums3), ans);
    }

    {
        let nums1 = vec![3, 1];
        let nums2 = vec![2, 3];
        let nums3 = vec![1, 2];
        let ans = vec![1, 2, 3];
        assert_eq!(two_out_of_three(nums1, nums2, nums3), ans);
    }

    {
        let nums1 = vec![1, 2, 2];
        let nums2 = vec![4, 3, 3];
        let nums3 = vec![5];
        let ans = vec![];
        assert_eq!(two_out_of_three(nums1, nums2, nums3), ans);
    }
}
