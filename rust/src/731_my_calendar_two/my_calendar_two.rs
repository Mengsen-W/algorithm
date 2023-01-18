/*
 * @Date: 2022-07-19
 * @LastEditors: mengsenwang mengsen_wang@163.com
 * @LastEditTime: 2022-07-19
 * @FilePath: /algorithm/731_my_calendar_two/my_calendar_two.rs
 */

use std::collections::BTreeMap;

struct MyCalendarTwo {
    map: BTreeMap<(i32, i32), i32>,
}

impl MyCalendarTwo {
    fn new() -> Self {
        Self {
            map: Default::default(),
        }
    }

    fn book(&mut self, mut start: i32, mut end: i32) -> bool {
        if start == end {
            return true;
        }

        let iter1 = self.map.range(..=(start, i32::MAX)).rev().next();
        let mut removed = vec![];
        let mut inserted = vec![];

        if let Some((key, val)) = iter1 {
            let key = *key;
            let val = *val;

            if key.1 > end {
                if val >= 2 {
                    return false;
                }
                self.map.remove(&key);
                if key.0 < start {
                    self.map.insert((key.0, start), val);
                }
                self.map.insert((start, end), val + 1);
                self.map.insert((end, key.1), val);
                return true;
            } else if key.1 > start {
                if val >= 2 {
                    return false;
                }
                removed.push(key);
                if key.0 < start {
                    inserted.push(((key.0, start), val));
                }
                inserted.push(((start, key.1), val + 1));
                start = key.1;
            }
        }

        let vec: Vec<_> = self
            .map
            .range((start, 0)..)
            .take_while(|(key, _)| key.0 < end)
            .map(|(key, val)| (*key, *val))
            .collect();

        for (key, val) in vec.iter().rev() {
            if *val >= 2 {
                return false;
            }
            if key.0 < start {
                self.map.remove(&key);
                self.map.insert((key.0, start), *val);

                if end < key.1 {
                    self.map.insert((start, end), val + 1);
                    self.map.insert((end, key.1), *val);
                } else {
                    self.map.insert((start, key.1), val + 1);
                    self.map.insert((key.1, end), 1);
                }
                for key in removed {
                    self.map.remove(&key);
                }
                for (key, val) in inserted {
                    self.map.insert(key, val);
                }

                return true;
            } else {
                removed.push(*key);
                if end < key.1 {
                    inserted.push(((key.0, end), *val + 1));
                    inserted.push(((end, key.1), *val));
                } else {
                    inserted.push(((key.0, key.1), *val + 1));
                    inserted.push(((key.1, end), 1));
                }
                end = key.0;
            }
        }

        for key in removed {
            self.map.remove(&key);
        }
        for (key, val) in inserted {
            self.map.insert(key, val);
        }
        if start < end {
            self.map.insert((start, end), 1);
        }

        true
    }
}

fn main() {
    let mut m = MyCalendarTwo::new();
    assert!(m.book(10, 20)); // returns true
    assert!(m.book(50, 60)); // returns true
    assert!(m.book(10, 40)); // returns true
    assert!(!m.book(5, 15)); // returns false
    assert!(m.book(5, 10)); // returns true
    assert!(m.book(25, 55)); // returns true
}
