/*
 * @Date: 2021-09-03 13:22:08
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-09-03 13:47:34
 */

struct Solution;

impl Solution {
    pub fn smallest_k(mut arr: Vec<i32>, k: i32) -> Vec<i32> {
        let len = arr.len();
        if len <= 0 {
            return vec![];
        }
        Self::quick_select(&mut arr, 0, len - 1, k as usize);
        let mut ans = vec![];
        for i in 0..(k as usize) {
            ans.push(arr[i]);
        }
        return ans;
    }

    fn quick_select(arr: &mut Vec<i32>, start: usize, end: usize, k: usize) {
        if start >= end {
            return;
        }

        let pivot = arr[end];
        let mut i = start;
        let mut j = start;

        while j < end {
            if arr[j] < pivot {
                arr.swap(i, j);
                i += 1;
            }
            j += 1;
        }

        arr.swap(i, end);
        if i < k {
            Self::quick_select(arr, i + 1, end, k);
        } else {
            Self::quick_select(arr, start, i - 1, k);
        }
    }
}

fn main() {
    let arr = vec![1, 3, 5, 7, 2, 4, 6, 8];
    let k = 4;
    let ans = vec![1, 3, 2, 4];
    assert_eq!(Solution::smallest_k(arr, k), ans);
}
