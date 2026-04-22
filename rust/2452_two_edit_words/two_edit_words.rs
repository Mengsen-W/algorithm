struct Solution;

impl Solution {
    pub fn two_edit_words(queries: Vec<String>, dictionary: Vec<String>) -> Vec<String> {
        let mut ans = Vec::new();

        for query in &queries {
            for s in &dictionary {
                let mut dis = 0;
                for (c1, c2) in query.chars().zip(s.chars()) {
                    if c1 != c2 {
                        dis += 1;
                    }
                }
                if dis <= 2 {
                    ans.push(query.clone());
                    break;
                }
            }
        }

        ans
    }
}

fn main() {
    let tests = vec![
        (
            vec!["word", "note", "ants", "wood"],
            vec!["wood", "joke", "moat"],
            vec!["word", "note", "wood"],
        ),
        (vec!["yes"], vec!["not"], vec![]),
    ];

    for (queries, dictionary, ans) in tests {
        assert_eq!(
            Solution::two_edit_words(
                queries.iter().map(|s| s.to_string()).collect(),
                dictionary.iter().map(|s| s.to_string()).collect()
            ),
            ans
        );
    }
}
