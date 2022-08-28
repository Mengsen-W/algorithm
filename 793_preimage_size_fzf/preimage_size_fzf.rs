/*
 * @Date: 2022-08-28
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-08-28
 * @FilePath: /algorithm/793_preimage_size_fzf/preimage_size_fzf.rs
 */

pub fn preimage_size_fzf(k: i32) -> i32 {
    fn zeta(mut n: i64) -> i64 {
        let mut res = 0;
        while n > 0 {
            n /= 5;
            res += n;
        }
        res
    }
    fn bin_left_serch(k: i64) -> i32 {
        let mut i = 0;
        let mut j = k * 5;
        while i < j {
            let m = (i + j) >> 1;
            let z = zeta(m);
            if z < k {
                i = m + 1;
            } else {
                j = m;
            }
        }
        i as i32
    }
    let k = k as i64;
    bin_left_serch(k + 1) - bin_left_serch(k)
}

fn main() {
    assert_eq!(preimage_size_fzf(0), 5);
    assert_eq!(preimage_size_fzf(5), 0);
    assert_eq!(preimage_size_fzf(3), 5);
}
