/*
 * @Date: 2023-04-14
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-04-14
 * @FilePath: /algorithm/rust/1023_camel_match/camel_match.rs
 */

pub fn camel_match(queries: Vec<String>, pattern: String) -> Vec<bool> {
    fn is_similar(s: &str, p: &str) -> bool {
        let mut aiter = s.chars();
        let mut biter = p.chars();
        loop {
            let mut a = '_';
            while let Some(t) = aiter.next() {
                if t.is_ascii_uppercase() {
                    a = t;
                    break;
                }
            }
            let mut b = '_';
            while let Some(t) = biter.next() {
                if t.is_ascii_uppercase() {
                    b = t;
                    break;
                }
            }
            if a != b {
                return false;
            } else if a == '_' {
                break;
            }
        }
        let mut s = s.chars();
        for c in p.chars() {
            let mut ok = false;
            while let Some(t) = s.next() {
                if t == c {
                    ok = true;
                    break;
                }
            }
            if !ok {
                return false;
            }
        }
        true
    }
    queries
        .into_iter()
        .map(|q| is_similar(&q, &pattern))
        .collect()
}

fn main() {
    {
        let queries = vec![
            "FooBar",
            "FooBarTest",
            "FootBall",
            "FrameBuffer",
            "ForceFeedBack",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        let pattern = "FB".to_string();
        let ans = vec![true, false, true, true, false];
        assert_eq!(camel_match(queries, pattern), ans);
    }

    {
        let queries = vec![
            "FooBar",
            "FooBarTest",
            "FootBall",
            "FrameBuffer",
            "ForceFeedBack",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        let pattern = "FoBa".to_string();
        let ans = vec![true, false, true, false, false];
        assert_eq!(camel_match(queries, pattern), ans);
    }

    {
        let queries = vec![
            "FooBar",
            "FooBarTest",
            "FootBall",
            "FrameBuffer",
            "ForceFeedBack",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        let pattern = "FoBaT".to_string();
        let ans = vec![false, true, false, false, false];
        assert_eq!(camel_match(queries, pattern), ans);
    }
}
