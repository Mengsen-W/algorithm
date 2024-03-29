/*
 * @Date: 2023-05-04
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-05-04
 * @FilePath: /algorithm/rust/2106_max_total_fruits/max_total_fruits.rs
 */

struct Solution;

impl Solution {
    pub fn max_total_fruits(fruits: Vec<Vec<i32>>, start_pos: i32, k: i32) -> i32 {
        let n = fruits.len();
        let mut sum = vec![0; n + 1];
        let mut indices = vec![0; n];

        for i in 0..n {
            sum[i + 1] = sum[i] + fruits[i][1];
            indices[i] = fruits[i][0];
        }

        let mut ans = 0;

        for x in 0..=k / 2 {
            let y = k - 2 * x;
            let left = start_pos - x;
            let right = start_pos + y;
            let start = Self::lower_bound(&indices, 0, n - 1, left);
            let end = Self::upper_bound(&indices, 0, n - 1, right);
            ans = ans.max(sum[end] - sum[start]);

            let y = k - 2 * x;
            let left = start_pos - y;
            let right = start_pos + x;
            let start = Self::lower_bound(&indices, 0, n - 1, left);
            let end = Self::upper_bound(&indices, 0, n - 1, right);
            ans = ans.max(sum[end] - sum[start]);
        }

        ans
    }

    fn lower_bound(arr: &[i32], left: usize, right: usize, val: i32) -> usize {
        let mut res = right + 1;
        let (mut left, mut right) = (left, right);

        while left <= right {
            let mid = left + (right - left) / 2;

            if arr[mid] >= val {
                res = mid;

                if mid == 0 {
                    break;
                }

                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }

        res
    }

    fn upper_bound(arr: &[i32], left: usize, right: usize, val: i32) -> usize {
        let mut res = right + 1;
        let (mut left, mut right) = (left, right);

        while left <= right {
            let mid = left + (right - left) / 2;

            if arr[mid] > val {
                res = mid;

                if mid == 0 {
                    break;
                }

                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }

        res
    }
}

fn main() {
    {
        let fruits = vec![vec![2, 8], vec![6, 3], vec![8, 6]];
        let start_pos = 5;
        let k = 4;
        let ans = 9;
        assert_eq!(Solution::max_total_fruits(fruits, start_pos, k), ans);
    }

    {
        let fruits = vec![
            vec![0, 9],
            vec![4, 1],
            vec![5, 7],
            vec![6, 2],
            vec![7, 4],
            vec![10, 9],
        ];
        let start_pos = 5;
        let k = 4;
        let ans = 14;
        assert_eq!(Solution::max_total_fruits(fruits, start_pos, k), ans);
    }

    {
        let fruits = vec![vec![0, 3], vec![6, 4], vec![8, 5]];
        let start_pos = 3;
        let k = 2;
        let ans = 0;
        assert_eq!(Solution::max_total_fruits(fruits, start_pos, k), ans);
    }
}
