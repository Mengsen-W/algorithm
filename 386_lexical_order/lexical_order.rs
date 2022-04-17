/*
 * @Date: 2022-04-18 07:14:52
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-04-18 07:26:13
 * @FilePath: /algorithm/386_lexical_order/lexical_order.rs
 */

pub fn lexical_order(n: i32) -> Vec<i32> {
    let n = n as usize;
    let mut ans = vec![0; n];
    let mut num = 1;
    for i in 0..n {
        ans[i] = num;
        if num * 10 <= n {
            num *= 10;
        } else {
            while num % 10 == 9 || num + 1 > n {
                num /= 10;
            }
            num += 1;
        }
    }
    ans.iter().map(|&x| x as i32).collect()
}

fn main() {
    assert_eq!(
        lexical_order(13),
        vec![1, 10, 11, 12, 13, 2, 3, 4, 5, 6, 7, 8, 9]
    );

    assert_eq!(lexical_order(2), vec![1, 2]);
}
