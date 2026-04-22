/*
 * @Author: Mengsen.Wang
 * @Date: 2021-02-15 11:01:28
 * @Last Modified by: Mengsen.Wang
 * @Last Modified time: 2021-02-15 11:13:44
 */

fn find_max_consecutive_ones(nums: &Vec<i32>) -> i32 {
    nums.split(|&x| x == 0) // use `0` value to split vector into slices which only contains `1`
        .map(|v| v.iter().count()) // count each `1` slice's length
        .max() // find the max length
        // because split an empty vector will at least return an empty slice
        // which makes sense, so `max()` will always return `Some(T)`.
        // But the `max()` function signature returns `Option<T>`, we will use `unwrap_or(default)` for that.
        .unwrap_or(0) as i32
}

fn main() {
    let nums = vec![1, 0, 1, 1, 0, 1];
    assert_eq!(find_max_consecutive_ones(&nums), 2);
}
