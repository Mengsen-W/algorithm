/*
 * @Date: 2024-01-28
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-01-28
 * @FilePath: /algorithm/rust/366_can_measure_water/can_measure_water.rs
 */

struct Solution;

impl Solution {
    pub fn can_measure_water(x: i32, y: i32, z: i32) -> bool {
        if x + y < z {
            return false;
        }
        if x == 0 || y == 0 {
            return z == 0 || x + y == z;
        }

        fn gcd(x: i32, y: i32) -> i32 {
            if y == 0 {
                return x;
            }
            return gcd(y, x % y);
        }

        return z % gcd(x, y) == 0;
    }
}

fn main() {
    let tests = vec![(3, 5, 4, true), (2, 6, 5, false), (1, 2, 3, true)];

    for (x, y, z, ans) in tests {
        assert_eq!(Solution::can_measure_water(x, y, z), ans);
    }
}
