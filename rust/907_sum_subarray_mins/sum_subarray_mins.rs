/*
 * @Date: 2022-10-28
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-11-28
 * @FilePath: /algorithm/rust/907_sum_subarray_mins/sum_subarray_mins.rs
 */

struct Solution;

impl Solution {
    pub fn sum_subarray_mins(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let mut left = vec![0; n];
        let mut right = vec![0; n];
        let mut stack = vec![];

        for i in 0..n {
            left[i] = i + 1;
            right[i] = n - i;

            while let Some(j) = stack.pop() {
                if arr[j] < arr[i] {
                    stack.push(j);
                    break;
                } else {
                    right[j] = i - j;
                }
            }

            if let Some(&j) = stack.last() {
                left[i] = i - j;
            }

            stack.push(i);
        }

        let mut ans = 0;

        for i in 0..n {
            ans = (ans + left[i] as i64 * right[i] as i64 * arr[i] as i64) % 1_000_000_007;
        }
        ans as i32
    }
}

fn main() {
    let tests = vec![(vec![3, 1, 2, 4], 17), (vec![11, 81, 94, 43, 3], 444)];

    for (arr, ans) in tests {
        assert_eq!(Solution::sum_subarray_mins(arr), ans);
    }
}
