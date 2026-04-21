/*
 * @Date: 2022-08-25
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-08-25
 * @FilePath: /algorithm/658_find_closest_elements/find_closest_elements.rs
 */

pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
    let n = arr.len();
    let mut lo = 0;
    let mut hi = n - k as usize;

    while lo < hi {
        let mid = lo + (hi - lo) / 2;

        if 2 * x > arr[mid] + arr[mid + k as usize] {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }

    arr[lo..lo + k as usize].to_vec()
}

fn main() {
    {
        let arr = vec![1, 2, 3, 4, 5];
        let k = 4;
        let x = 3;
        let ans = vec![1, 2, 3, 4];
        assert_eq!(find_closest_elements(arr, k, x), ans);
    }

    {
        let arr = vec![1, 2, 3, 4, 5];
        let k = 4;
        let x = -1;
        let ans = vec![1, 2, 3, 4];
        assert_eq!(find_closest_elements(arr, k, x), ans);
    }
}
