/*
 * @Date: 2023-06-26
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-06-26
 * @FilePath: /algorithm/rust/2485_pivot_integer/pivot_integer.rs
 */

pub fn pivot_integer(n: i32) -> i32 {
    let t = (n * n + n) / 2;
    let x = (t as f64).sqrt() as i32;
    if x.pow(2) == t {
        x
    } else {
        -1
    }
}

fn main() {
    let test_map = vec![(8, 6), (1, 1), (4, -1)];
    for (n, expect) in test_map {
        assert_eq!(pivot_integer(n), expect);
    }
}
