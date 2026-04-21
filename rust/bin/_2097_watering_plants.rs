/*
 * @Date: 2024-05-08
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-05-08
 * @FilePath: /algorithm/rust/2097_watering_plants/watering_plants.rs
 */

struct Solution;

impl Solution {
    pub fn watering_plants(plants: Vec<i32>, capacity: i32) -> i32 {
        let mut ans = 0;
        let mut rest = capacity;
        for (i, &plant) in plants.iter().enumerate() {
            if rest >= plant {
                ans += 1;
                rest -= plant;
            } else {
                ans += i as i32 * 2 + 1;
                rest = capacity - plant;
            }
        }
        ans
    }
}

fn main() {
    let tests = vec![
        (vec![2, 2, 3, 3], 5, 14),
        (vec![1, 1, 1, 4, 2, 3], 4, 30),
        (vec![7, 7, 7, 7, 7, 7, 7], 8, 40),
    ];

    for (plants, capacity, ans) in tests {
        assert_eq!(Solution::watering_plants(plants, capacity), ans);
    }
}
