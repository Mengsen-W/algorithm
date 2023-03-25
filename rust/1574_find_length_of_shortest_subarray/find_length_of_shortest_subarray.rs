/*
 * @Date: 2023-03-25
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-03-25
 * @FilePath: /algorithm/rust/1574_find_length_of_shortest_subarray/find_length_of_shortest_subarray.rs
 */

pub fn find_length_of_shortest_subarray(arr: Vec<i32>) -> i32 {
    let n = arr.len();
    let mut left = 0;

    while left < n - 1 && arr[left] <= arr[left + 1] {
        left += 1;
    }

    if left == n - 1 {
        return 0;
    }

    let mut right = n - 1;

    while right > 0 && right < n && arr[right] >= arr[right - 1] {
        right -= 1;
    }

    let mut ans = (n - left - 1).min(right);

    let mut i = 0;
    let mut j = right;

    while i <= left && j <= n - 1 {
        if arr[i] <= arr[j] {
            ans = ans.min(j - i - 1);
            i += 1;
        } else {
            j += 1;
        }
    }

    ans as i32
}

fn main() {
    {
        let arr = vec![1, 2, 3, 10, 4, 2, 3, 5];
        let ans = 3;
        assert_eq!(find_length_of_shortest_subarray(arr), ans);
    }

    {
        let arr = vec![5, 4, 3, 2, 1];
        let ans = 4;
        assert_eq!(find_length_of_shortest_subarray(arr), ans);
    }

    {
        let arr = vec![1, 2, 3];
        let ans = 0;
        assert_eq!(find_length_of_shortest_subarray(arr), ans);
    }

    {
        let arr = vec![1];
        let ans = 0;
        assert_eq!(find_length_of_shortest_subarray(arr), ans);
    }
}
