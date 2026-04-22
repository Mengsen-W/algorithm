/*
 * @Date: 2023-02-09
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-02-09
 * @FilePath: /algorithm/rust/1797_AuthenticationManager/AuthenticationManager.rs
 */

use std::collections::HashMap;

struct AuthenticationManager {
    tab: HashMap<String, i32>,
    ttl: i32,
}

impl AuthenticationManager {
    fn new(time_to_live: i32) -> Self {
        Self {
            tab: HashMap::new(),
            ttl: time_to_live,
        }
    }

    fn generate(&mut self, token_id: String, current_time: i32) {
        *self.tab.entry(token_id).or_insert(0) = current_time + self.ttl;
    }

    fn renew(&mut self, token_id: String, current_time: i32) {
        if let Some(t) = self.tab.get_mut(&token_id) {
            if *t > current_time {
                *t = current_time + self.ttl;
            }
        }
    }

    fn count_unexpired_tokens(&self, current_time: i32) -> i32 {
        self.tab
            .iter()
            .fold(0, |ans, (_, &t)| ans + (t > current_time) as i32)
    }
}

fn main() {
    let mut a = AuthenticationManager::new(5);
    a.renew(String::from("aaa"), 1);
    a.generate(String::from("aaa"), 2);
    assert_eq!(a.count_unexpired_tokens(6), 1);
    a.generate(String::from("bbb"), 7);
    a.renew(String::from("aaa"), 8);
    a.renew(String::from("bbb"), 10);
    assert_eq!(a.count_unexpired_tokens(15), 0);
}
