struct Solution;

impl Solution {
    pub fn brace_expansion_ii(expression: String) -> Vec<String> {
        use std::collections::BTreeSet;
        let mut s: Vec<(BTreeSet<String>, Vec<String>)> = Vec::new();
        let mut b: BTreeSet<String> = BTreeSet::new();
        let mut c: Vec<String> = vec!["".to_string()];
        for ch in expression.chars() {
            match ch {
                '{' => {
                    s.push((b, c));
                    b = BTreeSet::new();
                    c = vec!["".to_string()];
                }
                '}' => {
                    if let Some((bb, cc)) = s.pop() {
                        b.extend(c.drain(..));
                        c = cc
                            .into_iter()
                            .flat_map(|l| b.iter().map(move |r| format!("{}{}", l, r)))
                            .collect();
                        b = bb;
                    }
                }
                ',' => {
                    b.extend(c.drain(..));
                    c = vec!["".to_string()];
                }
                cc => {
                    c.iter_mut().for_each(|v| v.push(cc));
                }
            }
        }
        b.extend(c.drain(..));
        b.into_iter().collect()
    }
}

fn main() {
    let tests = vec![
        ("{a,b}{c,{d,e}}", vec!["ac", "ad", "ae", "bc", "bd", "be"]),
        ("{{a,z},a{b,c},{ab,z}}", vec!["a", "ab", "ac", "z"]),
    ];

    for (expression, ans) in tests {
        assert_eq!(
            Solution::brace_expansion_ii(expression.to_string()),
            ans.iter().map(|s| s.to_string()).collect::<Vec<_>>()
        );
    }
}
