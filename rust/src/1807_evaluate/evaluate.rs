/*
 * @Date: 2023-01-12
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-01-12
 * @FilePath: /algorithm/1807_evaluate/evaluate.rs
 */

pub fn evaluate(s: String, knowledge: Vec<Vec<String>>) -> String {
    use std::collections::HashMap;
    let mut ans = String::new();
    let knowledge = knowledge
        .into_iter()
        .map(|v| (v[0].to_owned(), v[1].to_owned()))
        .collect::<HashMap<String, String>>();

    let mut flag = false;
    let mut cur = String::new();
    for ch in s.chars() {
        match ch {
            '(' => flag = true,
            ')' => {
                ans.push_str(knowledge.get(&cur).unwrap_or(&"?".to_string()));
                cur.clear();
                flag = false;
            }
            _ => {
                if flag {
                    cur.push(ch);
                } else {
                    ans.push(ch);
                }
            }
        }
    }
    ans
}

fn main() {
    {
        let s = String::from("(name)is(age)yearsold");
        let knowledge = [["name", "bob"], ["age", "two"]]
            .iter()
            .map(|v| v.iter().map(|s| s.to_string()).collect())
            .collect();
        let ans = String::from("bobistwoyearsold");
        assert_eq!(evaluate(s, knowledge), ans);
    }

    {
        let s = String::from("hi(name)");
        let knowledge = [["a", "b"]]
            .iter()
            .map(|v| v.iter().map(|s| s.to_string()).collect())
            .collect();
        let ans = String::from("hi?");
        assert_eq!(evaluate(s, knowledge), ans);
    }

    {
        let s = String::from("(a)(a)(a)aaa");
        let knowledge = [["a", "yes"]]
            .iter()
            .map(|v| v.iter().map(|s| s.to_string()).collect())
            .collect();
        let ans = String::from("yesyesyesaaa");
        assert_eq!(evaluate(s, knowledge), ans);
    }
}
