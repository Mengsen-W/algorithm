/*
 * @Date: 2023-02-07
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-02-07
 * @FilePath: /algorithm/rust/1604_alert_names/alert_names.rs
 */

pub fn alert_names(key_name: Vec<String>, key_time: Vec<String>) -> Vec<String> {
    fn to_mins(time: &String) -> i32 {
        let time: Vec<&str> = time.split(':').collect();
        let (a, b): (_, _) = (
            time[0].parse::<i32>().unwrap(),
            time[1].parse::<i32>().unwrap(),
        );
        a * 60 + b
    }
    use std::collections::HashMap;

    let mut map = HashMap::new();
    let n = key_name.len();

    for i in 0..n {
        let count = map.entry(&key_name[i]).or_insert(Vec::new());
        count.push(to_mins(&key_time[i]));
    }

    let mut ans: Vec<String> = Vec::new();
    for (key, val) in map.iter_mut() {
        val.sort_unstable();
        let vn = val.len();
        if vn < 3 {
            continue;
        }
        for i in 0..vn - 2 {
            if val[i + 2] - val[i] <= 60 {
                ans.push((*key).clone());
                break;
            }
        }
    }
    ans.sort_unstable();
    ans
}

fn main() {
    {
        let key_name = vec!["daniel", "daniel", "daniel", "luis", "luis", "luis", "luis"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let key_time = vec![
            "10:00", "10:40", "11:00", "09:00", "11:00", "13:00", "15:00",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        let ans: Vec<String> = vec!["daniel"].iter().map(|s| s.to_string()).collect();
        assert_eq!(alert_names(key_name, key_time), ans);
    }

    {
        let key_name = vec!["alice", "alice", "alice", "bob", "bob", "bob", "bob"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let key_time = vec![
            "12:01", "12:00", "18:00", "21:00", "21:20", "21:30", "23:00",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        let ans: Vec<String> = vec!["bob"].iter().map(|s| s.to_string()).collect();
        assert_eq!(alert_names(key_name, key_time), ans);
    }

    {
        let key_name = vec!["john", "john", "john"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let key_time = vec!["23:58", "23:59", "00:01"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let ans: Vec<String> = Vec::new();
        assert_eq!(alert_names(key_name, key_time), ans);
    }

    {
        let key_name = vec![
            "leslie", "leslie", "leslie", "clare", "clare", "clare", "clare",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        let key_time = vec![
            "13:00", "13:20", "14:00", "18:00", "18:51", "19:30", "19:49",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        let ans: Vec<String> = vec!["clare", "leslie"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(alert_names(key_name, key_time), ans);
    }
}
