/*
 * @Date: 2022-05-31 10:03:34
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-05-31 10:49:09
 * @FilePath: /algorithm/114_alien_order/alien_order.rs
 */

pub fn alien_order(words: Vec<String>) -> String {
    use std::collections::HashMap;
    let mut edges: HashMap<char, Vec<char>> = HashMap::new();
    let mut indegrees: HashMap<char, i32> = HashMap::new();
    let mut valid = true;
    fn add_edge(
        edges: &mut std::collections::HashMap<char, Vec<char>>,
        indegrees: &mut std::collections::HashMap<char, i32>,
        valid: &mut bool,
        before: &String,
        after: &String,
    ) {
        let length_1 = before.len();
        let length_2 = after.len();
        let length = std::cmp::min(length_1, length_2);
        let mut index = 0;
        while index < length {
            let c1 = before.chars().nth(index).unwrap();
            let c2 = after.chars().nth(index).unwrap();
            if c1 != c2 {
                edges.entry(c1).or_insert(vec![]).push(c2);
                use std::ops::AddAssign;
                indegrees.entry(c2).or_insert(0).add_assign(1);
                break;
            }
            index += 1;
        }
        if index == length && length_1 > length_2 {
            *valid = false;
        }
    }

    let length = words.len();
    for word in &words {
        let word_length = word.len();
        for j in 0..word_length {
            let c = word.chars().nth(j).unwrap();
            if !edges.contains_key(&c) {
                edges.insert(c, vec![]);
            }
        }
    }

    let mut i = 1;
    while i < length && valid {
        add_edge(
            &mut edges,
            &mut indegrees,
            &mut valid,
            &words[i - 1],
            &words[i],
        );
        i += 1;
    }
    if !valid {
        return String::new();
    }
    use std::collections::VecDeque;
    let mut queue: VecDeque<char> = edges
        .iter()
        .filter(|&(u, _)| !indegrees.contains_key(u))
        .map(|(u, _)| *u)
        .collect();

    let mut order: String = String::new();
    while let Some(u) = queue.pop_front() {
        order.push(u);
        if let Some(v) = edges.get(&u) {
            for c in v {
                use std::ops::SubAssign;
                indegrees.entry(*c).and_modify(|e| e.sub_assign(1));
                if indegrees.get(c).unwrap() == &0 {
                    queue.push_back(*c);
                }
            }
        }
    }

    match order.len() != edges.len() {
        true => String::new(),
        false => order,
    }
}

fn main() {
    assert_eq!(
        alien_order(
            vec!["wrt", "wrf", "er", "ett", "rftt"]
                .iter()
                .map(|&s| s.to_string())
                .collect()
        ),
        String::from("wertf")
    );
    assert_eq!(
        alien_order(vec!["z", "x"].iter().map(|&s| String::from(s)).collect()),
        String::from("zx")
    );
    assert_eq!(
        alien_order(
            vec!["z", "x", "z"]
                .iter()
                .map(|&s| String::from(s))
                .collect()
        ),
        String::new()
    );
}
