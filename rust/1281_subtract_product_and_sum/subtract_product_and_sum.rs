/*
 * @Date: 2023-08-09
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-08-09
 * @FilePath: /algorithm/rust/1281_subtract_product_and_sum/subtract_product_and_sum.rs
 */

struct Solution;
impl Solution {
    pub fn subtract_product_and_sum(mut n: i32) -> i32 {
        let mut x = 1;
        let mut y = 0;
        while n != 0 {
            let v = n % 10;
            n /= 10;
            x *= v;
            y += v;
        }
        x - y
    }
}

fn main() {
    let tests = vec![(234, 15), (4421, 21)];

    for (n, ans) in tests {
        assert_eq!(Solution::subtract_product_and_sum(n), ans);
    }
}
