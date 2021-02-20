/*
 * @Author: Mengsen.Wang
 * @Date: 2021-02-20 09:07:11
 * @Last Modified by: Mengsen.Wang
 * @Last Modified time: 2021-02-20 09:15:51
 */

use std::collections::HashMap;

fn find_shortest_sub_array(nums: Vec<i32>) -> i32 {
    let mut cnt_left_len = HashMap::new();
    let mut degree = 0;

    for i in 0..nums.len() {
        let cll = cnt_left_len.entry(nums[i]).or_insert([0, i, 1]);
        cll[0] += 1;
        cll[2] = i - cll[1] + 1;
        degree = degree.max(cll[0])
    }

    cnt_left_len
        .values()
        .filter(|arr| arr[0] == degree)
        .map(|arr| arr[2])
        .min()
        .unwrap() as i32
}

fn main() {
    let mut nums: Vec<i32>;
    nums = vec![1, 2, 2, 3, 1];
    assert_eq!(find_shortest_sub_array(nums.clone()), 2);
    nums = vec![1, 2, 2, 3, 1];
    assert_eq!(find_shortest_sub_array(nums.clone()), 2);
    nums = vec![1, 2, 2, 3];
    assert_eq!(find_shortest_sub_array(nums.clone()), 2);
    nums = vec![2, 2, 3, 1];
    assert_eq!(find_shortest_sub_array(nums.clone()), 2);
    nums = vec![1, 2, 2];
    assert_eq!(find_shortest_sub_array(nums.clone()), 2);
    nums = vec![2, 2, 3];
    assert_eq!(find_shortest_sub_array(nums.clone()), 2);
    nums = vec![2, 2];
    assert_eq!(find_shortest_sub_array(nums.clone()), 2);
    nums = vec![1, 2, 2, 3, 1, 4, 2];
    assert_eq!(find_shortest_sub_array(nums.clone()), 6);
}
