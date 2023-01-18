/*
 * @Date: 2022-04-13 09:17:15
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-04-13 09:20:10
 * @FilePath: /algorithm/380_randomized_set/randomized_set.rs
 */

use std::collections::HashMap;

struct RandomizedSet {
    nums: Vec<i32>,
    map: HashMap<i32, usize>,
}

impl RandomizedSet {
    fn new() -> Self {
        Self {
            nums: vec![],
            map: HashMap::new(),
        }
    }

    fn insert(&mut self, val: i32) -> bool {
        if self.map.get(&val).is_some() {
            return false;
        }
        self.map.insert(val, self.nums.len());
        self.nums.push(val);
        true
    }

    fn remove(&mut self, val: i32) -> bool {
        if let Some(index) = self.map.get(&val) {
            let &last = self.nums.last().unwrap();
            self.nums[*index] = last;
            *self.map.get_mut(&last).unwrap() = *index;
            self.nums.pop().unwrap();
            self.map.remove(&val);
            return true;
        }
        false
    }

    fn get_random(&self) -> i32 {
        let x = rand::random::<usize>() % self.nums.len();
        self.nums[x]
    }
}

fn main() {
    let randomized_set = RandomizedSet::new();
    assert_eq!(randomized_set.insert(1), true);
    assert_eq!(randomized_set.remove(2), false);
    assert_eq!(randomized_set.insert(2), true);
    assert_eq!(
        randomized_set.get_random() == 1 || randomized_set.get_random() == 2,
        true
    );
    assert_eq!(randomized_set.remove(1), true);
    assert_eq!(randomized_set.insert(2), false);
    assert_eq!(randomized_set.get_random() == 2, true);
}
