/*
 * @Date: 2022-10-20
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-10-20
 * @FilePath: /algorithm/779_kth_grammar/kth_grammar.rs
 */

pub fn kth_grammar(n: i32, k: i32) -> i32 {
    // (k - 1).count_ones() as i32 & 1

    let mut k = k - 1;
    let mut res = 0;
    while (k > 0) {
        k &= k - 1;
        res ^= 1;
    }
    res
}

fn main() {
    assert_eq!(kth_grammar(1, 1), 0);
    assert_eq!(kth_grammar(2, 1), 0);
    assert_eq!(kth_grammar(2, 2), 1);
}
