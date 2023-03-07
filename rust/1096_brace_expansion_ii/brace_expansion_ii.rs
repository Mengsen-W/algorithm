/*
 * @Date: 2023-03-07
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-03-07
 * @FilePath: /algorithm/rust/1096_brace_expansion_ii/brace_expansion_ii.rs
 */

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

fn main() {
    {
        let expression = String::from("{a,b}{c,{d,e}}");
        let ans: Vec<String> = vec!["ac", "ad", "ae", "bc", "bd", "be"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(brace_expansion_ii(expression), ans);
    }

    {
        let expression = String::from("{{a,z},a{b,c},{ab,z}}");
        let ans: Vec<String> = vec!["a", "ab", "ac", "z"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(brace_expansion_ii(expression), ans);
    }
}
