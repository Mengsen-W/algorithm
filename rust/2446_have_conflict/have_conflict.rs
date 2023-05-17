/*
 * @Date: 2023-05-17
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-05-17
 * @FilePath: /algorithm/rust/2446_have_conflict/have_conflict.rs
 */

pub fn have_conflict(event1: Vec<String>, event2: Vec<String>) -> bool {
    event1[0] <= event2[1] && event1[1] >= event2[0]
}

fn main() {
    {
        let event1 = vec!["01:15", "02:00"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let event2 = vec!["02:00", "03:00"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let ans = true;
        assert_eq!(have_conflict(event1, event2), ans);
    }

    {
        let event1 = vec!["01:00", "02:00"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let event2 = vec!["01:20", "03:00"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let ans = true;
        assert_eq!(have_conflict(event1, event2), ans);
    }

    {
        let event1 = vec!["10:00", "11:00"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let event2 = vec!["14:00", "15:00"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let ans = false;
        assert_eq!(have_conflict(event1, event2), ans);
    }
}
