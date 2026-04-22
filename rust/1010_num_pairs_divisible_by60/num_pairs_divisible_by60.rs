/*
 * @Date: 2023-05-07
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-05-07
 * @FilePath: /algorithm/rust/1010_num_pairs_divisible_by60/num_pairs_divisible_by60.rs
 */

pub fn num_pairs_divisible_by60(time: Vec<i32>) -> i64 {
    let helper = |x| (x * (x - 1)) >> 1;
    let mut tab = [0; 60];
    time.into_iter().for_each(|t| tab[t as usize % 60] += 1i64);
    let cnt = (1..30).fold(0, |cnt, i| cnt + tab[i] * tab[60 - i]);
    cnt + helper(tab[0]) + helper(tab[30])
}

fn main() {
    {
        let time = vec![30, 20, 150, 100, 40];
        let ans = 3;
        assert_eq!(num_pairs_divisible_by60(time), ans);
    }

    {
        let time = vec![60, 60, 60];
        let ans = 3;
        assert_eq!(num_pairs_divisible_by60(time), ans);
    }
}
