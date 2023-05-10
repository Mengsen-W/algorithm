/*
 * @Date: 2023-05-10
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-05-10
 * @FilePath: /algorithm/rust/1015_smallest_repunit_div_by_k/smallest_repunit_div_by_k.rs
 */

pub fn smallest_repunit_div_by_k(k: i32) -> i32 {
    if k % 2 == 0 || k % 5 == 0 {
        return -1;
    }
    let mut temp = 1;
    let mut len = 1;
    while temp % k != 0 {
        temp = temp % k;
        temp = temp * 10 + 1;
        len += 1;
    }
    len as i32
}

fn main() {
    assert_eq!(smallest_repunit_div_by_k(1), 1);
    assert_eq!(smallest_repunit_div_by_k(2), -1);
    assert_eq!(smallest_repunit_div_by_k(3), 3);
}
