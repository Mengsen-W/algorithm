/*
 * @Date: 2023-11-23
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-11-23
 * @FilePath: /algorithm/rust/1410_entity_parser/entity_parser.rs
 */

struct Solution;

impl Solution {
    pub fn entity_parser(text: String) -> String {
        let tab = [
            ("&quot;".to_string().into_bytes(), b'"'),
            ("&apos;".to_string().into_bytes(), b'\''),
            ("&amp;".to_string().into_bytes(), b'&'),
            ("&gt;".to_string().into_bytes(), b'>'),
            ("&lt;".to_string().into_bytes(), b'<'),
            ("&frasl;".to_string().into_bytes(), b'/'),
        ];
        let text = text.into_bytes();
        let mut ans = Vec::with_capacity(text.len());
        let mut i = 0;
        while i < text.len() {
            if text[i] == b'&' {
                let mut ok = false;
                for t in tab.iter() {
                    let len = t.0.len();
                    if i + len <= text.len() {
                        if text[i..i + len] == t.0 {
                            ans.push(t.1);
                            i += len;
                            ok = true;
                            break;
                        }
                    }
                }
                if ok {
                    continue;
                }
                ans.push(b'&');
                i += 1;
            } else {
                ans.push(text[i]);
                i += 1;
            }
        }
        unsafe { String::from_utf8_unchecked(ans) }
    }
}

fn main() {
    let tests = vec![
        (
            "&amp; is an HTML entity but &ambassador; is not.",
            "& is an HTML entity but &ambassador; is not.",
        ),
        ("and I quote: &quot;...&quot;", "and I quote: \"...\""),
        (
            "Stay home! Practice on Leetcode :)",
            "Stay home! Practice on Leetcode :)",
        ),
        (
            "x &gt; y &amp;&amp; x &lt; y is always false",
            "x > y && x < y is always false",
        ),
        (
            "leetcode.com&frasl;problemset&frasl;all",
            "leetcode.com/problemset/all",
        ),
    ];

    for (text, ans) in tests {
        assert_eq!(Solution::entity_parser(text.to_string()), ans.to_string());
    }
}
