/*
 * @Date: 2021-07-03 10:14:15
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-07-03 11:38:37
 */

fn frequency_sort(s: String) -> String {
    use std::collections::HashMap;
    let mut mp = HashMap::new();
    let mut max_freq = s.len();
    s.chars().for_each(|c| {
        *mp.entry(c).or_insert(0) += 1;
        max_freq = max_freq.max(*(mp.get(&c).unwrap()) as usize);
    });
    let mut buckets: Vec<String> = vec![String::new(); max_freq + 1];
    mp.iter()
        .for_each(|(key, value)| buckets[*value as usize].push(*key));
    let mut ret = String::new();
    for i in (1..=max_freq).rev() {
        let bucket = &buckets[i];
        bucket.chars().for_each(|ch| {
            (0..i).for_each(|_| ret.push(ch));
        });
    }
    ret
}

fn main() {
    let s = "tree".to_string();
    let ans = frequency_sort(s);
    assert!(ans == "eetr".to_string() || ans == "eert".to_string());
    let s = "cccaaa".to_string();
    let ans = frequency_sort(s);
    assert!(ans == "cccaaa".to_string() || ans == "aaaccc".to_string());
    let s = "Aabb".to_string();
    let ans = frequency_sort(s);
    assert!(ans == "bbaA".to_string() || ans == "bbAa".to_string());
}
