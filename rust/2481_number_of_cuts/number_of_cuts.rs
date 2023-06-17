/*
 * @Date: 2023-06-17
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-06-17
 * @FilePath: /algorithm/rust/2481_number_of_cuts/number_of_cuts.rs
 */

pub fn number_of_cuts(n: i32) -> i32 {
    match n {
        1 => 0,
        n if n % 2 == 0 => n / 2,
        _ => n,
    }
}

fn main() {
    assert_eq!(number_of_cuts(4), 2);
    assert_eq!(number_of_cuts(3), 3);
}
