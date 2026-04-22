struct Solution;

impl Solution {
    pub fn distinct_names(ideas: Vec<String>) -> i64 {
        use std::collections::{HashMap, HashSet};
        let mut names: HashMap<char, HashSet<String>> = HashMap::new();

        for idea in ideas.iter() {
            let first_char = idea.chars().next().unwrap();
            let suffix = idea[1..].to_string();
            names
                .entry(first_char)
                .or_insert_with(HashSet::new)
                .insert(suffix);
        }

        let get_intersect_size =
            |a: &HashSet<String>, b: &HashSet<String>| -> usize { a.intersection(b).count() };

        let mut ans: i64 = 0;
        for (pre_a, set_a) in names.iter() {
            for (pre_b, set_b) in names.iter() {
                if pre_a == pre_b {
                    continue;
                }
                let intersect = get_intersect_size(set_a, set_b);
                ans += (set_a.len() - intersect) as i64 * (set_b.len() - intersect) as i64;
            }
        }
        ans
    }
}

fn main() {
    let tests = vec![
        (vec!["coffee", "donuts", "time", "toffee"], 6),
        (vec!["lack", "back"], 0),
    ];

    for (ideas, ans) in tests {
        assert_eq!(
            Solution::distinct_names(ideas.into_iter().map(|x| x.to_string()).collect()),
            ans
        );
    }
}
