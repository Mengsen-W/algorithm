struct Solution;

impl Solution {
    pub fn minimum_operations_to_make_k_periodic(word: String, k: i32) -> i32 {
        use std::collections::HashMap;
        let n = word.len();
        let k = k as usize;
        let mut cnt = HashMap::new();
        for i in (k..=n).step_by(k) {
            *cnt.entry(&word[i - k..i]).or_insert(0) += 1;
        }
        let mx = *cnt.values().max().unwrap();
        (n / k) as i32 - mx
    }
}

fn main() {
    let tests = vec![("leetcodeleet", 4, 1), ("leetcoleet", 2, 3)];

    for (word, k, ans) in tests {
        assert_eq!(
            Solution::minimum_operations_to_make_k_periodic(word.to_string(), k),
            ans
        );
    }
}
