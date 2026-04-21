/*
 * @Date: 2023-04-22
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-04-22
 * @FilePath: /algorithm/rust/1027_longest_arith_seq_length/longest_arith_seq_length.rs
 */

pub fn longest_arith_seq_length(nums: Vec<i32>) -> i32 {
    use std::collections::HashMap;
    let mut ans = 0;
    let mut map = HashMap::new();
    for j in 0..nums.len() {
        for i in 0..j {
            let d = nums[j] - nums[i];
            let cur = match map.get(&(i, d)) {
                Some(&cnt) => cnt + 1,
                None => 2,
            };
            map.insert((j, d), cur);
            ans = ans.max(cur);
        }
    }
    ans
}

fn main() {
    {
        let nums = vec![3, 6, 9, 12];
        let ans = 4;
        assert_eq!(longest_arith_seq_length(nums), ans);
    }

    {
        let nums = vec![9, 4, 7, 2, 10];
        let ans = 3;
        assert_eq!(longest_arith_seq_length(nums), ans);
    }

    {
        let nums = vec![20, 1, 15, 3, 10, 5, 8];
        let ans = 4;
        assert_eq!(longest_arith_seq_length(nums), ans);
    }
}
