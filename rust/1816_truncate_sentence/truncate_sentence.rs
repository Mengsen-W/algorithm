/*
 * @Date: 2021-12-06 02:36:20
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-12-06 02:45:42
 */

pub fn truncate_sentence(s: String, k: i32) -> String {
    let v: Vec<&str> = s.split(' ').collect();
    let mut ret = String::new();
    let l = v.len();
    let l = l.min(k as usize);
    if l == v.len() {
        return s;
    } else {
        for i in 0..l {
            ret.push_str(v[i]);
            if i != (l - 1) {
                ret.push(' ');
            }
        }
    }
    ret
}

fn main() {
    assert_eq!(
        truncate_sentence("Hello how are you Contestant".to_string(), 4),
        "Hello how are you".to_string()
    );
    assert_eq!(
        truncate_sentence("What is the solution to this problem".to_string(), 4),
        "What is the solution".to_string()
    );
    assert_eq!(
        truncate_sentence("chopper is not a tanuki".to_string(), 5),
        "chopper is not a tanuki".to_string()
    );
}
