/*
 * @Date: 2022-10-31
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-10-31
 * @FilePath: /algorithm/481_magical_string/magical_string.rs
 */

pub fn magical_string(n: i32) -> i32 {
    let mut sign = true;
    let mut signs = vec![];
    (0..n as usize).fold(0, |mut ans, i| {
        signs.push(sign);

        if signs[i] {
            ans += 1;
        } else {
            signs.push(sign);
        }

        sign = !sign;
        ans
    })
}

fn main() {
    assert_eq!(magical_string(6), 3);
    assert_eq!(magical_string(1), 1);
}
