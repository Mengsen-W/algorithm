/*
 * @Date: 2022-11-13
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-11-13
 * @FilePath: /algorithm/791_custom_sort_string/custom_sort_string.rs
 */

pub fn custom_sort_string(order: String, s: String) -> String {
    let mut tab = [26; 26];
    order.bytes().enumerate().for_each(|(i, c)| {
        tab[c as usize - 'a' as usize] = i;
    });
    let mut s: Vec<char> = s.chars().collect();
    s.sort_unstable_by(|a, b| {
        usize::cmp(
            &tab[*a as usize - 'a' as usize],
            &tab[*b as usize - 'a' as usize],
        )
    });
    s.into_iter().collect()
}

fn main() {
    {
        let order = String::from("cba");
        let s = String::from("abcd");
        let ans = String::from("cbad");
        assert_eq!(custom_sort_string(order, s), ans)
    }

    {
        let order = String::from("cbafg");
        let s = String::from("abcd");
        let ans = String::from("cbad");
        assert_eq!(custom_sort_string(order, s), ans)
    }
}
