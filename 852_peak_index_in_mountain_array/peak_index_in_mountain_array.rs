/*
 * @Date: 2021-06-15 08:40:25
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-06-15 09:04:08
 */

fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
    let n = arr.len();
    let mut left = 1;
    let mut right = n - 2;
    let mut ans = 0;

    while left <= right {
        let mid = (left + right) / 2;
        if arr[mid] > arr[mid + 1] {
            ans = mid;
            right = mid - 1;
        } else {
            left = mid + 1;
        }
    }
    ans as i32
}

fn main() {
    {
        let arr = vec![0, 1, 0];
        let ans = 1;
        assert_eq!(peak_index_in_mountain_array(arr), ans);
    }
    {
        let arr = vec![0, 2, 1, 0];
        let ans = 1;
        assert_eq!(peak_index_in_mountain_array(arr), ans);
    }
    {
        let arr = vec![0, 10, 5, 2, 0];
        let ans = 1;
        assert_eq!(peak_index_in_mountain_array(arr), ans);
    }
    {
        let arr = vec![3, 4, 5, 1];
        let ans = 2;
        assert_eq!(peak_index_in_mountain_array(arr), ans);
    }
    {
        let arr = vec![24, 69, 100, 99, 79, 78, 67, 36, 26, 19];
        let ans = 2;
        assert_eq!(peak_index_in_mountain_array(arr), ans);
    }
}
