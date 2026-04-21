/*
 * @Date: 2023-05-12
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-05-12
 * @FilePath: /algorithm/rust/1016_query_string/query_string.rs
 */

pub fn query_string(s: String, n: i32) -> bool {
    for i in (n / 2 + 1..=n).rev() {
        if !s.contains(format!("{:b}", i).as_str()) {
            return false;
        }
    }
    true
}

fn main() {
    {
        let s = "0110".to_string();
        let n = 3;
        let ans = true;
        assert_eq!(query_string(s, n), ans);
    }

    {
        let s = "0110".to_string();
        let n = 4;
        let ans = false;
        assert_eq!(query_string(s, n), ans);
    }
}
