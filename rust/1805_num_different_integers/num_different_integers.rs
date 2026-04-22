/*
 * @Date: 2022-12-06
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-12-06
 * @FilePath: /algorithm/1805_num_different_integers/num_different_integers.rs
 */

pub fn num_different_integers(word: String) -> i32 {
    word.split(|c: char| !c.is_ascii_digit())
        .filter(|s| !s.is_empty())
        .map(|s| s.trim_start_matches('0'))
        .collect::<std::collections::HashSet<_>>()
        .len() as i32
}

fn main() {
    {
        let word = String::from("a123bc34d8ef34");
        let ans = 3;
        assert_eq!(num_different_integers(word), ans);
    }

    {
        let word = String::from("leet1234code234");
        let ans = 2;
        assert_eq!(num_different_integers(word), ans);
    }

    {
        let word = String::from("a1b01c001");
        let ans = 1;
        assert_eq!(num_different_integers(word), ans);
    }
}
