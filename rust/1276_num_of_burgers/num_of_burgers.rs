/*
 * @Date: 2023-12-25
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-12-25
 * @FilePath: /algorithm/rust/1276_num_of_burgers/num_of_burgers.rs
 */

struct Solution;

impl Solution {
    pub fn num_of_burgers(tomato_slices: i32, cheese_slices: i32) -> Vec<i32> {
        if tomato_slices % 2 != 0
            || tomato_slices < cheese_slices * 2
            || cheese_slices * 4 < tomato_slices
        {
            return vec![];
        }
        return vec![
            tomato_slices / 2 - cheese_slices,
            cheese_slices * 2 - tomato_slices / 2,
        ];
    }
}

fn main() {
    let tests = vec![
        (16, 7, vec![1, 6]),
        (17, 4, vec![]),
        (4, 17, vec![]),
        (2, 1, vec![0, 1]),
    ];

    for (tomato_slices, cheese_slices, ans) in tests {
        assert_eq!(Solution::num_of_burgers(tomato_slices, cheese_slices), ans);
    }
}
