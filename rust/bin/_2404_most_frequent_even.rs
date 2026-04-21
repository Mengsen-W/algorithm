/*
 * @Date: 2023-04-13
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-04-14
 * @FilePath: /algorithm/rust/2404_most_frequent_even/most_frequent_even.rs
 */

pub fn most_frequent_even(nums: Vec<i32>) -> i32 {
    use std::collections::BTreeMap;
    let mut cnt = BTreeMap::new();
    nums.into_iter().for_each(|num| {
        if num % 2 == 0 {
            *cnt.entry(num).or_default() += 1
        }
    });
    cnt.into_iter()
        .rev()
        .max_by_key(|&e| e.1)
        .unwrap_or((-1, -1))
        .0
}

fn main() {
    {
        let nums = vec![0, 1, 2, 2, 4, 4, 1];
        let ans = 2;
        assert_eq!(most_frequent_even(nums), ans);
    }

    {
        let nums = vec![4, 4, 4, 9, 2, 4];
        let ans = 4;
        assert_eq!(most_frequent_even(nums), ans);
    }

    {
        let nums = vec![29, 47, 21, 41, 13, 37, 25, 7];
        let ans = -1;
        assert_eq!(most_frequent_even(nums), ans);
    }
}
