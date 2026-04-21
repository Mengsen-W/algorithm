/*
 * @Date: 2021-12-29 01:26:57
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-12-29 01:46:45
 */

pub fn count_quadruplets(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    use std::collections::HashMap;
    let mut map: HashMap<_, _> = HashMap::new();
    let mut ans = 0;
    for b in (1..=n - 3).rev() {
        for d in b + 2..n {
            *map.entry(nums[d] - nums[b + 1]).or_insert(0) += 1;
        }
        for a in 0..b {
            let sum = nums[a] + nums[b];
            if map.contains_key(&sum) {
                ans += map[&sum];
            }
        }
    }
    return ans;
}

fn main() {
    assert_eq!(count_quadruplets(vec![1, 2, 3, 6]), 1);
    assert_eq!(count_quadruplets(vec![3, 3, 6, 4, 5]), 0);
    assert_eq!(count_quadruplets(vec![1, 1, 1, 3, 5]), 4);
}
