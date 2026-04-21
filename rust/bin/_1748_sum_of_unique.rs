/*
 * @Date: 2022-02-06 02:18:25
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-02-06 02:54:54
 */

pub fn sum_of_unique(nums: Vec<i32>) -> i32 {
    use std::collections::HashMap;
    let mut map: HashMap<i32, i32> = HashMap::new();
    let mut ans = 0;
    for num in nums {
        match map.get_mut(&num) {
            Some(v) => {
                if *v == 0 {
                    *v = 1;
                    ans += num;
                } else if *v == 1 {
                    *v = 2;
                    ans -= num;
                }
            }
            None => {
                map.insert(num, 1);
                ans += num;
            }
        }
    }
    ans
}

fn main() {
    assert_eq!(sum_of_unique(vec![1, 2, 3, 2]), 4);
    assert_eq!(sum_of_unique(vec![1, 1, 1, 1, 1]), 0);
    assert_eq!(sum_of_unique(vec![1, 2, 3, 4, 5]), 15);
}
