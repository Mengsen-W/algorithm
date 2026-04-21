/*
 * @Date: 2023-10-20
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-10-20
 * @FilePath: /algorithm/rust/2525_categorize_box/categorize_box.rs
 */

struct Solution;

impl Solution {
    pub fn categorize_box(length: i32, width: i32, height: i32, mass: i32) -> String {
        const BOX_TYPE: [&str; 4] = ["Neither", "Bulky", "Heavy", "Both"];
        let bulky = [length, width, height].iter().any(|&x| x >= 10i32.pow(4))
            || [length, width, height]
                .iter()
                .map(|&x| x as i64)
                .product::<i64>()
                >= 10i64.pow(9);
        let heavy = mass >= 100;
        BOX_TYPE[bulky as usize + heavy as usize * 2].to_string()
    }
}

fn main() {
    let tests = vec![(1000, 35, 700, 300, "Heavy"), (200, 50, 800, 50, "Neither")];
    for (length, width, height, mass, ans) in tests {
        assert_eq!(Solution::categorize_box(length, width, height, mass), ans);
    }
}
