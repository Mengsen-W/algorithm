/*
 * @Date: 2022-01-31 02:47:43
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-01-31 03:12:12
 */

pub fn number_of_steps(mut num: i32) -> i32 {
    if num == 0 {
        return 0;
    }
    let mut ans = 0;
    while num > 1 {
        ans += if num & 1 == 1 { 2 } else { 1 };
        num /= 2;
    }
    ans + 1
}

pub fn number_of_steps2(num: i32) -> i32 {
    match num {
        0 => 0,
        _ => (2 * num.count_ones() + num.count_zeros() - num.leading_zeros() - 1) as i32,
    }
}

fn main() {
    assert_eq!(number_of_steps(14), 6);
    assert_eq!(number_of_steps(8), 4);
    assert_eq!(number_of_steps(123), 12);
    assert_eq!(number_of_steps2(14), 6);
    assert_eq!(number_of_steps2(8), 4);
    assert_eq!(number_of_steps2(123), 12);
}
