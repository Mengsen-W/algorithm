/*
 * @Date: 2021-06-25 09:00:29
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-06-25 09:29:05
 */

use std::{
    collections::{HashSet, VecDeque},
    iter::FromIterator,
};

pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
    let dead_set: HashSet<String> = HashSet::from_iter(deadends);
    let start_str = String::from("0000");
    if dead_set.contains(&start_str) || dead_set.contains(&target) {
        return -1;
    }
    let mut rst = 0;
    if start_str == target {
        return rst;
    }
    let mut visited_set: HashSet<String> = HashSet::from_iter(vec![start_str.clone()]);
    let mut end_deque = VecDeque::from(vec![target]);
    let mut start_deque = VecDeque::from(vec![start_str]);
    let mut deque = start_deque.clone();
    let mut prev_set: HashSet<String> = HashSet::from_iter(end_deque.clone());
    let mut forword = true;
    while deque.len() > 0 {
        let mut count = deque.len();
        rst += 1;
        while count > 0 {
            if let Some(v) = deque.pop_front() {
                for i in 0..8 {
                    let s = change_str(v.clone(), i / 2, i % 2 == 1);
                    if prev_set.contains(&s) {
                        return rst;
                    }
                    if !dead_set.contains(&s) && !visited_set.contains(&s) {
                        visited_set.insert(s.clone());
                        deque.push_back(s);
                    }
                }
            }
            count -= 1;
        }
        if forword {
            start_deque = deque;
            prev_set = HashSet::from_iter(start_deque.clone());
            deque = end_deque.clone();
        } else {
            end_deque = deque;
            prev_set = HashSet::from_iter(end_deque.clone());
            deque = start_deque.clone();
        }
        forword = !forword;
    }
    -1
}

pub fn change_str(s: String, i: usize, add: bool) -> String {
    let mut cs: Vec<char> = s.chars().collect();
    let num = cs[i].to_digit(10);
    if let Some(mut v) = num {
        if add {
            if v == 9 {
                v = 0;
            } else {
                v += 1;
            }
        } else {
            if v == 0 {
                v = 9;
            } else {
                v -= 1;
            }
        }
        cs[i] = v.to_string().chars().next().unwrap();
    }
    cs.iter().collect()
}

fn main() {
    {
        let deadends = vec![
            "0201".to_string(),
            "0101".to_string(),
            "0102".to_string(),
            "1212".to_string(),
            "2002".to_string(),
        ];
        let target = "0202".to_string();
        assert_eq!(open_lock(deadends, target), 6);
    }
    {
        let deadends = vec!["8888".to_string()];
        let target = "0009".to_string();
        assert_eq!(open_lock(deadends, target), 1);
    }
    {
        let deadends = vec![
            "8887".to_string(),
            "8889".to_string(),
            "8878".to_string(),
            "8898".to_string(),
            "8788".to_string(),
            "8988".to_string(),
            "7888".to_string(),
            "9888".to_string(),
        ];
        let target = "8888".to_string();
        assert_eq!(open_lock(deadends, target), -1);
    }
    {
        let deadends = vec!["0000".to_string()];
        let target = "8888".to_string();
        assert_eq!(open_lock(deadends, target), -1);
    }
}
