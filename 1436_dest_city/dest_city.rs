/*
 * @Date: 2021-10-01 09:39:35
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-10-01 10:10:53
 */

struct Solution;
impl Solution {
    pub fn dest_city(paths: Vec<Vec<String>>) -> String {
        use std::collections::HashSet;
        let mut cities: HashSet<String> = HashSet::new();
        for path in &paths {
            cities.insert(path[0].clone());
        }
        for path in &paths {
            if !cities.contains(&path[1]) {
                return path[1].clone();
            }
        }
        String::new()
    }
}

fn main() {
    {
        let paths: Vec<Vec<String>> = [
            ["London", "New York"],
            ["New York", "Lima"],
            ["Lima", "Sao Paulo"],
        ]
        .iter()
        .map(|path| path.iter().map(|s| s.to_string()).collect())
        .collect();
        let ans = "Sao Paulo".to_string();
        assert_eq!(Solution::dest_city(paths), ans);
    }
    {
        let paths: Vec<Vec<String>> = [["B", "C"], ["D", "B"], ["C", "A"]]
            .iter()
            .map(|path| path.iter().map(|s| s.to_string()).collect())
            .collect();
        let ans = "A".to_string();
        assert_eq!(Solution::dest_city(paths), ans);
    }
}
