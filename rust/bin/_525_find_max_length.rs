/*
 * @Date: 2021-06-03 08:19:48
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-06-03 08:43:46
 */

fn find_max_length(nums: Vec<i32>) -> i32 {
    use std::collections::HashMap;
    let mut map: HashMap<i32, i32> = HashMap::new();
    map.insert(0, -1);
    let (mut sum, mut ans) = (0, 0);
    let n = nums.len();
    for i in 0..n {
        if nums[i] == 0 {
            sum += -1;
        } else {
            sum += 1;
        }
        if map.contains_key(&sum) {
            ans = ans.max(i as i32 - map.get(&sum).unwrap());
        } else {
            map.insert(sum, i as i32);
        }
    }
    ans as i32
}

fn main() {
    assert_eq!(find_max_length(vec![0, 1]), 2);
    assert_eq!(find_max_length(vec![0, 1, 0]), 2);
}
