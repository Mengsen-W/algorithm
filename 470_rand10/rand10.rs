/*
 * @Date: 2021-09-05 10:08:00
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-09-05 10:12:05
 */

fn rand7() -> i32 {
    0
}

impl Solution {
    pub fn rand10() -> i32 {
        let mut a = rand7();
        let mut b = rand7() * 7;

        while a + b > 47 {
            a = rand7();
            b = rand7() * 7;
        }

        (a + b + 2) % 10 + 1
    }
}
