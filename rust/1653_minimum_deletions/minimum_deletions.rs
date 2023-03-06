/*
 * @Date: 2023-03-06
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-03-06
 * @FilePath: /algorithm/rust/1653_minimum_deletions/minimum_deletions.rs
 */

pub fn minimum_deletions(s: String) -> i32 {
    let s: Vec<char> = s.chars().collect();
    let (mut leftb, mut righta) = (0, 0);
    for i in 0..s.len() {
        if s[i] == 'a' {
            righta += 1;
        }
    }
    let mut res = righta;
    for i in 0..s.len() {
        let c = s[i];
        if c == 'a' {
            righta -= 1;
        } else {
            leftb += 1;
        }
        res = res.min(leftb + righta);
    }
    res
}

fn main() {
    {
        let s = String::from("aababbab");
        let ans = 2;
        assert_eq!(minimum_deletions(s), ans);
    }

    {
        let s = String::from("bbaaaaabb");
        let ans = 2;
        assert_eq!(minimum_deletions(s), ans);
    }
}
