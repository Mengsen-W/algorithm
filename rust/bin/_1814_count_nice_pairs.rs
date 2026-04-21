/*
 * @Date: 2023-01-17
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-01-17
 * @FilePath: /algorithm/1814_count_nice_pairs/count_nice_pairs.rs
 */

pub fn count_nice_pairs(nums: Vec<i32>) -> i32 {
    use std::collections::HashMap;
    nums.into_iter()
        .fold((0, HashMap::new()), |(mut ans, mut map), num| {
            let (mut temp, mut rev_num) = (num, 0);
            while temp > 0 {
                rev_num = rev_num * 10 + temp % 10;
                temp /= 10;
            }
            let dif = num - rev_num;
            if let Some(v) = map.get_mut(&dif) {
                ans = (ans + *v) % (1e9 as i32 + 7);
                *v += 1;
            } else {
                map.insert(dif, 1);
            }
            (ans, map)
        })
        .0
}

fn main() {
    {
        let nums = vec![42, 11, 1, 97];
        let ans = 2;
        assert_eq!(count_nice_pairs(nums), ans);
    }

    {
        let nums = vec![13, 10, 35, 24, 76];
        let ans = 4;
        assert_eq!(count_nice_pairs(nums), ans);
    }
}
