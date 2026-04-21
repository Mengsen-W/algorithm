/*
 * @Date: 2021-12-31 01:05:31
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-12-31 01:23:25
 */

pub fn check_perfect_number(num: i32) -> bool {
    if num == 1 {
        return false;
    }

    let mut d = 2;
    let mut sum = 1;

    while d * d <= num {
        if num % d == 0 {
            sum += d;
            if d * d < num {
                sum += num / d;
            }
        }
        d += 1;
    }
    sum == num
}

fn main() {
    assert_eq!(check_perfect_number(28), true);
    assert_eq!(check_perfect_number(6), true);
    assert_eq!(check_perfect_number(496), true);
    assert_eq!(check_perfect_number(8128), true);
    assert_eq!(check_perfect_number(2), false);
}
