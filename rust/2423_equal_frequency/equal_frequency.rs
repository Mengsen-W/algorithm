/*
 * @Date: 2023-04-29
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-04-29
 * @FilePath: /algorithm/rust/2423_equal_frequency/equal_frequency.rs
 */

pub fn equal_frequency(word: String) -> bool {
    let b: Vec<_> = word.bytes().collect();
    for i in 0..b.len() {
        let mut cnt = std::collections::HashMap::new();
        b[..i].iter().for_each(|c| {
            let v = cnt.entry(*c).or_insert(0);
            *v += 1;
        });
        b[i + 1..].iter().for_each(|c| {
            let v = cnt.entry(*c).or_insert(0);
            *v += 1;
        });
        if cnt.values().collect::<std::collections::HashSet<_>>().len() == 1 {
            return true;
        }
    }
    false
}

fn main() {
    assert_eq!(equal_frequency("abcc".to_string()), true);
    assert_eq!(equal_frequency("aazz".to_string()), false);
}
