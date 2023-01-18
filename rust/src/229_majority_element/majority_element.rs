/*
 * @Date: 2021-10-22 00:48:12
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-10-22 01:18:36
 * @FilePath: /algorithm/229_majority_element/majority_element.rs
 * @Description: file content
 */

struct Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        use std::collections::HashMap;
        let n = nums.len() as i32;
        let mut ans: Vec<i32> = Vec::new();
        let mut cnt: HashMap<i32, i32> = HashMap::new();

        nums.iter().for_each(|x| (*cnt.entry(*x).or_insert(0)) += 1);
        cnt.iter().for_each(|(key, value)| {
            if value > &(n / 3) {
                ans.push(*key)
            }
        });
        ans
    }
}

fn main() {
    assert_eq!(Solution::majority_element(vec![3, 2, 3]), vec![3]);
    assert_eq!(Solution::majority_element(vec![1]), vec![1]);
    assert_eq!(
        Solution::majority_element(vec![1, 1, 1, 3, 3, 2, 2, 2]),
        vec![2, 1]
    );
}
