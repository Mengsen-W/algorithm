/*
 * @Author: Mengsen.Wang
 * @Date: 2021-01-28 20:30:07
 * @Last Modified by: Mengsen.Wang
 * @Last Modified time: 2021-01-30 23:02:06
 */

use std::collections::HashMap;

fn min_window(s: &Vec<u8>, t: &Vec<u8>) -> String {
    let mut need: HashMap<u8, i32> = HashMap::new();
    let mut window: HashMap<u8, i32> = HashMap::new();
    "".to_string()
}

fn check_inclusion(s: &String, t: &String) -> bool {
    false
}

fn find_anagrams(s: &String, t: &String) -> Vec<i32> {
    vec![1, 2, 3]
}

fn length_of_longest_substring(s: &String) -> i32 {
    0
}

fn main() {
    min_window(
        &"ADOBECODEBANC".to_string().into_bytes(),
        &"ABC".to_string().into_bytes(),
    );
}
