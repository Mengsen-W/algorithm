/*
 * @Date: 2021-05-20 08:49:00
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-05-20 09:35:32
 */

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(Eq)]
struct WordFrequency {
    word: String,
    frequency: i32,
}

impl Ord for WordFrequency {
    fn cmp(&self, other: &Self) -> Ordering {
        self.frequency
            .cmp(&other.frequency)
            .then(other.word.cmp(&self.word))
    }
}

impl PartialOrd for WordFrequency {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for WordFrequency {
    fn eq(&self, other: &Self) -> bool {
        self.frequency == other.frequency && self.word == other.word
    }
}

fn top_k_frequent(words: Vec<String>, k: i32) -> Vec<String> {
    let mut map = HashMap::new();
    words.into_iter().for_each(|word| {
        *map.entry(word).or_insert(0) += 1;
    });
    let mut heap = BinaryHeap::new();
    map.into_iter().for_each(|(word, frequency)| {
        heap.push(WordFrequency { word, frequency });
    });
    let mut result = Vec::with_capacity(k as usize);
    for _ in 0..k {
        if let Some(WordFrequency { word, .. }) = heap.pop() {
            result.push(word);
        } else {
            break;
        }
    }
    result
}

fn main() {
    assert_eq!(
        top_k_frequent(
            vec![
                "i".to_string(),
                "love".to_string(),
                "leetcode".to_string(),
                "i".to_string(),
                "love".to_string(),
                "coding".to_string()
            ],
            2
        ),
        vec!["i", "love"]
    );
    assert_eq!(
        top_k_frequent(
            vec![
                "the".to_string(),
                "day".to_string(),
                "is".to_string(),
                "sunny".to_string(),
                "the".to_string(),
                "the".to_string(),
                "the".to_string(),
                "sunny".to_string(),
                "is".to_string(),
                "is".to_string(),
            ],
            4
        ),
        vec!["the", "is", "sunny", "day"]
    );
}
