/*
 * @Date: 2021-07-18 16:52:42
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-07-18 19:38:58
 */

fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    use std::collections::HashMap;
    let mut mp: HashMap<[i32; 26], Vec<String>> = HashMap::new();

    strs.iter().for_each(|s| {
        let mut cnt: [i32; 26] = [0; 26];
        let s: Vec<char> = s.chars().collect();
        s.iter().for_each(|b| cnt[(*b as u8 - b'a') as usize] += 1);
        mp.entry(cnt)
            .or_insert(vec![])
            .push(s.iter().collect::<String>());
    });
    let mut ans: Vec<Vec<String>> = Vec::new();
    for val in mp.values() {
        ans.push(val.clone());
    }
    ans
}

fn group_anagrams_2(strs: Vec<String>) -> Vec<Vec<String>> {
    use std::collections::HashMap;
    let mut mp: HashMap<u64, Vec<String>> = HashMap::new();
    let mut pf: [u64; 26] = [1; 26];
    const B: u64 = 97755331;

    for i in 1..26 {
        pf[i] = pf[i - 1].wrapping_mul(B);
    }

    strs.iter().for_each(|t| {
        let mut hash: u64 = 0;
        let t: Vec<char> = t.chars().collect();
        t.iter().for_each(|c| {
            let c = *c as u8;
            hash += pf[(c - b'a') as usize].wrapping_mul(c as u64);
        });
        mp.entry(hash)
            .or_insert(vec![])
            .push(t.iter().collect::<String>());
    });

    let mut ans: Vec<Vec<String>> = Vec::new();
    for val in mp.values() {
        ans.push(val.clone());
    }
    ans
}

fn main() {
    let strs = vec![
        "eat".to_string(),
        "tea".to_string(),
        "tan".to_string(),
        "ate".to_string(),
        "nat".to_string(),
        "bat".to_string(),
    ];
    println!("{:?}", group_anagrams(strs.clone()));
    println!("{:?}", group_anagrams_2(strs.clone()));
}
