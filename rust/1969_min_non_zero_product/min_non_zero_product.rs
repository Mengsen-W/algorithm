/*
 * @Date: 2024-03-20
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-03-20
 * @FilePath: /algorithm/rust/1969_min_non_zero_product/min_non_zero_product.rs
 */

struct Solution;

impl Solution {
    pub fn min_non_zero_product(p: i32) -> i32 {
        fn fast_pow(mut x: i64, mut n: i64, mod_val: i64) -> i64 {
            let mod_val = mod_val as i64;
            let mut res = 1;
            while n != 0 {
                if n & 1 == 1 {
                    res = (res * x) % mod_val;
                }
                x = (x * x) % mod_val;
                n >>= 1;
            }
            res
        }

        if p == 1 {
            return 1;
        }
        let mod_val = 1_000_000_007;
        let x = fast_pow(2, p as i64, mod_val) - 1;
        let y = 1i64 << (p - 1);
        (fast_pow(x - 1, y - 1, mod_val) * x % mod_val) as i32
    }
}

fn main() {
    let tests = vec![(1, 1), (2, 6), (3, 1512)];

    for (p, ans) in tests {
        assert_eq!(Solution::min_non_zero_product(p), ans);
    }
}
