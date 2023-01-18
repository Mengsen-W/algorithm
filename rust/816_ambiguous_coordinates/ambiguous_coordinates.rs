/*
 * @Date: 2022-11-07
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-11-07
 * @FilePath: /algorithm/816_ambiguous_coordinates/ambiguous_coordinates.rs
 */

pub fn ambiguous_coordinates(mut s: String) -> Vec<String> {
    fn verify(s: &str, i: usize) -> Option<String> {
        let (x, y) = if i == 0 { (s, "") } else { (&s[..i], &s[i..]) };
        let a: i32 = x.parse().unwrap();
        let b: i32 = y.parse().unwrap_or(i32::MAX);
        if a == 0 && x.len() != 1 {
            None
        } else if a != 0 && &x[..1] == "0" {
            None
        } else if b == 0 {
            None
        } else if !y.is_empty() && &y[y.len() - 1..] == "0" {
            None
        } else {
            if y == "" {
                Some(s.to_string())
            } else {
                Some(format!("{}.{}", x, y))
            }
        }
    }
    let mut ans = Vec::new();
    s.pop();
    for k in 2..s.len() {
        let (a, b) = (&s[1..k], &s[k..]);
        for i in 0..a.len() {
            if let Some(x) = verify(a, i) {
                for j in 0..b.len() {
                    if let Some(y) = verify(b, j) {
                        ans.push(format!("({}, {})", x, y));
                    }
                }
            }
        }
    }
    ans
}

fn main() {
    assert_eq!(
        ambiguous_coordinates(String::from("(123)")),
        vec!["(1, 23)", "(1, 2.3)", "(12, 3)", "(1.2, 3)"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
    );

    assert_eq!(
        ambiguous_coordinates(String::from("(00011)")),
        vec!["(0, 0.011)", "(0.001, 1)"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
    );

    assert_eq!(
        ambiguous_coordinates(String::from("(0123)")),
        vec![
            "(0, 123)",
            "(0, 1.23)",
            "(0, 12.3)",
            "(0.1, 23)",
            "(0.1, 2.3)",
            "(0.12, 3)"
        ]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
    );

    assert_eq!(
        ambiguous_coordinates(String::from("(100)")),
        vec!["(10, 0)"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
    );
}
