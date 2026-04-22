/*
 * @Date: 2022-10-29
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-10-29
 * @FilePath: /algorithm/1773_count_matches/count_matches.rs
 */

pub fn count_matches(items: Vec<Vec<String>>, rule_key: String, rule_value: String) -> i32 {
    items.iter().fold(0, |cnt, item| {
        (item[match &rule_key as &str {
            "type" => 0,
            "color" => 1,
            "name" => 2,
            _ => 3,
        }] == rule_value) as i32
            + cnt
    })
}

fn main() {
    {
        let items = [
            ["phone", "blue", "pixel"],
            ["computer", "silver", "lenovo"],
            ["phone", "gold", "iphone"],
        ]
        .iter()
        .map(|v| v.iter().map(|s| s.to_string()).collect())
        .collect();
        let rule_key = "color".to_string();
        let rule_value = "silver".to_string();
        let ans = 1;

        assert_eq!(count_matches(items, rule_key, rule_value), ans);
    }

    {
        let items = [
            ["phone", "blue", "pixel"],
            ["computer", "silver", "phone"],
            ["phone", "gold", "iphone"],
        ]
        .iter()
        .map(|v| v.iter().map(|s| s.to_string()).collect())
        .collect();
        let rule_key = "type".to_string();
        let rule_value = "phone".to_string();
        let ans = 2;

        assert_eq!(count_matches(items, rule_key, rule_value), ans);
    }
}
