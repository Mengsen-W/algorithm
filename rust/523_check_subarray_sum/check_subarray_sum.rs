/*
 * @Date: 2021-06-02 07:45:06
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-06-02 08:00:59
 */

fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
    use std::collections::HashMap;
    let size = nums.len();
    let mut map: HashMap<i32, i32> = HashMap::new();
    map.insert(0, -1);
    let mut rem = 0;

    for i in 0..size {
        rem = (rem + nums[i]) % k;
        if map.contains_key(&rem) {
            let pos = map[&rem];
            if (i as i32 - pos) >= 2 {
                return true;
            }
        } else {
            map.insert(rem, i as i32);
        }
    }
    false
}

fn main() {
    assert!(check_subarray_sum(vec![23, 2, 4, 6, 7], 6));
    assert!(check_subarray_sum(vec![23, 2, 6, 4, 7], 6));
    assert!(!check_subarray_sum(vec![23, 2, 6, 4, 7], 13));
}
