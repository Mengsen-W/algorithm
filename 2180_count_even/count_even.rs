/*
 * @Date: 2023-01-06
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-01-06
 * @FilePath: /algorithm/2180_count_even/count_even.rs
 */

pub fn count_even(num: i32) -> i32 {
    let (x, mut y) = (num % 10, num / 10);
    let (mut res, mut y_sum) = (y * 5, 0);
    while y != 0 {
        y_sum += y % 10;
        y /= 10;
    }
    if y_sum % 2 == 0 {
        res += x / 2 + 1;
    } else {
        res += (x + 1) / 2;
    }
    res - 1
}

fn main() {
    {
        let num = 4;
        let ans = 2;
        assert_eq!(count_even(num), ans);
    }

    {
        let num = 30;
        let ans = 14;
        assert_eq!(count_even(num), ans);
    }
}
