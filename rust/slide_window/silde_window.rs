/*
 * @Author: Mengsen.Wang
 * @Date: 2021-01-28 20:30:07
 * @Last Modified by: Mengsen.Wang
 * @Last Modified time: 2021-01-30 23:02:06
 */

use std::cmp;
use std::collections::HashMap;

fn min_window(s: &String, t: &String) -> String {
    let s: Vec<u8> = s.clone().into_bytes();
    let t: Vec<u8> = t.clone().into_bytes();
    let mut need: HashMap<u8, i32> = HashMap::new();
    let mut window: HashMap<u8, i32> = HashMap::new();
    t.iter().for_each(|ch| {
        need.entry(*ch).and_modify(|e| *e += 1).or_insert(1);
    });
    let mut left: usize = 0;
    let mut right: usize = 0;
    let mut valid: usize = 0;
    let mut start: usize = 0;
    let mut len: usize = std::usize::MAX;

    while right < s.len() {
        let c: u8 = s[right];
        right += 1;
        if need.contains_key(&c) {
            window.entry(c).and_modify(|e| *e += 1).or_insert(1);
            if window.get(&c) == need.get(&c) {
                valid += 1;
            }
        }

        while valid == need.len() {
            if right - left < len {
                start = left;
                len = right - left;
            }
            let d: u8 = s[left];
            left += 1;

            if need.contains_key(&d) {
                if window.get(&d) == need.get(&d) {
                    valid -= 1;
                }
                window.entry(d).and_modify(|e| *e -= 1);
            }
        }
    }

    if len == usize::MAX {
        String::new()
    } else {
        String::from_utf8(s[start..start + len].to_vec()).unwrap()
    }
}

fn check_inclusion(t: &String, s: &String) -> bool {
    let s: Vec<u8> = s.clone().into_bytes();
    let t: Vec<u8> = t.clone().into_bytes();
    let mut need: HashMap<u8, i32> = HashMap::new();
    let mut window: HashMap<u8, i32> = HashMap::new();
    t.iter().for_each(|ch| {
        need.entry(*ch).and_modify(|e| *e += 1).or_insert(1);
    });
    let mut left: usize = 0;
    let mut right: usize = 0;
    let mut valid: usize = 0;

    while right < s.len() {
        let c: u8 = s[right];
        right += 1;
        if need.contains_key(&c) {
            window.entry(c).and_modify(|e| *e += 1).or_insert(1);
            if window.get(&c) == need.get(&c) {
                valid += 1;
            }
        }

        while right - left >= t.len() {
            if valid == need.len() {
                return true;
            }
            let d: u8 = s[left];
            left += 1;

            if need.contains_key(&d) {
                if window.get(&d) == need.get(&d) {
                    valid -= 1;
                }
                window.entry(d).and_modify(|e| *e -= 1);
            }
        }
    }

    false
}

fn find_anagrams(s: &String, t: &String) -> Vec<usize> {
    let s: Vec<u8> = s.clone().into_bytes();
    let t: Vec<u8> = t.clone().into_bytes();
    let mut need: HashMap<u8, i32> = HashMap::new();
    let mut window: HashMap<u8, i32> = HashMap::new();
    t.iter().for_each(|ch| {
        need.entry(*ch).and_modify(|e| *e += 1).or_insert(1);
    });
    let mut left: usize = 0;
    let mut right: usize = 0;
    let mut valid: usize = 0;
    let mut res: Vec<usize> = vec![];

    while right < s.len() {
        let c: u8 = s[right];
        right += 1;
        if need.contains_key(&c) {
            window.entry(c).and_modify(|e| *e += 1).or_insert(1);
            if window.get(&c) == need.get(&c) {
                valid += 1;
            }
        }

        while right - left >= t.len() {
            if valid == need.len() {
                res.push(left);
            }
            let d: u8 = s[left];
            left += 1;

            if need.contains_key(&d) {
                if window.get(&d) == need.get(&d) {
                    valid -= 1;
                }
                window.entry(d).and_modify(|e| *e -= 1);
            }
        }
    }

    res
}

fn length_of_longest_substring(s: &String) -> usize {
    let s: Vec<u8> = s.clone().into_bytes();
    let mut window: HashMap<u8, i32> = HashMap::new();
    let mut left: usize = 0;
    let mut right: usize = 0;
    let mut res: usize = 0;

    while right < s.len() {
        let c: u8 = s[right];
        right += 1;
        window.entry(c).and_modify(|e| *e += 1).or_insert(1);
        while window.get(&c).unwrap() > &1 {
            let d: u8 = s[left];
            left += 1;
            window.entry(d).and_modify(|e| *e -= 1);
        }
        res = cmp::max(right - left, res);
    }
    res
}

fn main() {
    println!(
        "{:#?}",
        min_window(&"ADOBECODEBANC".to_string(), &"ABC".to_string())
    );
    println!(
        "{:#?}",
        min_window(&"EBBANCF".to_string(), &"ABC".to_string())
    );

    println!(
        "{:#?}",
        check_inclusion(&"ab".to_string(), &"eidbaooo".to_string())
    );
    println!(
        "{:#?}",
        check_inclusion(&"ab".to_string(), &"eidboaoo".to_string())
    );

    println!(
        "{:#?}",
        find_anagrams(&"cbaebabacd".to_string(), &"abc".to_string())
    );

    println!("{}", length_of_longest_substring(&"abcabcbb".to_string()));
    println!("{}", length_of_longest_substring(&"bbbbb".to_string()));
    println!("{}", length_of_longest_substring(&"pwwkew".to_string()));
}
