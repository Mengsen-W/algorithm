/*
 * @Date: 2022-11-30
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-11-30
 * @FilePath: /algorithm/895_FreqStack/FreqStack.rs
 */

#[derive(Default)]
pub struct FreqStack {
    freq: std::collections::HashMap<i32, usize>,
    group: Vec<Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FreqStack {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn push(&mut self, val: i32) {
        let entry = self.freq.entry(val).or_insert(0);
        if *entry >= self.group.len() {
            self.group.push(vec![]);
        }
        self.group[*entry].push(val);
        *entry += 1;
    }

    pub fn pop(&mut self) -> i32 {
        if let Some(max_freq) = self.group.last_mut() {
            if let Some(res) = max_freq.pop() {
                *self.freq.entry(res).or_default() -= 1;
                if max_freq.is_empty() {
                    self.group.pop();
                }
                return res;
            }
        }
        unreachable!()
    }
}

fn main() {
    let mut f = FreqStack::new();
    f.push(5);
    f.push(7);
    f.push(5);
    f.push(7);
    f.push(4);
    f.push(5);
    assert_eq!(f.pop(), 5);
    assert_eq!(f.pop(), 7);
    assert_eq!(f.pop(), 5);
    assert_eq!(f.pop(), 4);
}
