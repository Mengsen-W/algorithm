/*
 * @Date: 2023-01-10
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-01-10
 * @FilePath: /algorithm/753_crack_safe/crack_safe.rs
 */

pub fn crack_safe(n: i32, k: i32) -> String {
    use std::collections::HashMap;
    let mut seen: HashMap<i32, bool> = HashMap::new();
    let mut ans = String::new();
    let highest = 10_i32.pow((n - 1) as u32);

    fn dfs(node: i32, ans: &mut String, seen: &mut HashMap<i32, bool>, highest: &i32, k: &i32) {
        for x in 0..*k {
            let nei = node * 10 + x;
            if !seen.contains_key(&nei) {
                seen.insert(nei, true);
                dfs(nei % highest, ans, seen, highest, k);
                ans.push_str(&x.to_string());
            }
        }
    }

    dfs(0, &mut ans, &mut seen, &highest, &k);
    for _ in 1..n {
        ans.push('0');
    }
    ans
}

fn main() {
    {
        let n = 1;
        let k = 2;
        let ans = "10".to_string();
        assert_eq!(crack_safe(n, k), ans);
    }

    {
        let n = 2;
        let k = 2;
        let ans = "01100".to_string();
        assert_eq!(crack_safe(n, k), ans);
    }
}
