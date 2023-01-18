/*
 * @Date: 2022-10-28
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-10-28
 * @FilePath: /algorithm/907_sum_subarray_mins/sum_subarray_mins.rs
 */

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

fn main() {
    {
        let arr = vec![3, 1, 2, 4];
        let ans = 17;
        assert_eq!(sum_subarray_mins(arr), ans);
    }

    {
        let arr = vec![11, 81, 94, 43, 3];
        let ans = 444;
        assert_eq!(sum_subarray_mins(arr), ans);
    }
}
