/*
 * @Date: 2023-09-29
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-09-29
 * @FilePath: /algorithm/rust/605_can_place_flowers/can_place_flowers.rs
 */

struct Solution;
impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        let m = flowerbed.len();
        let mut n = n;
        let mut i = 0;
        while i < m {
            if (i == 0 || flowerbed[i - 1] == 0)
                && flowerbed[i] == 0
                && (i == m - 1 || flowerbed[i + 1] == 0)
            {
                n -= 1;
                i += 2; // 下一个位置肯定不能种花，直接跳过
            } else {
                i += 1;
            }
        }
        n <= 0
    }
}

fn main() {
    let tests = vec![
        (vec![1, 0, 0, 0, 1], 1, true),
        (vec![1, 0, 0, 0, 1], 2, false),
    ];

    for (flowerbed, n, ans) in tests {
        assert_eq!(Solution::can_place_flowers(flowerbed, n), ans);
    }
}
