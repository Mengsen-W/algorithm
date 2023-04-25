/*
 * @Date: 2023-04-25
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-04-25
 * @FilePath: /algorithm/rust/2418_sort_people/sort_people.rs
 */

pub fn sort_people(names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
    let mut cache = heights
        .iter()
        .zip(names.into_iter())
        .map(|(&k, v)| (k, v))
        .collect::<Vec<_>>();
    cache.sort_unstable_by_key(|(h, _)| -h);
    cache.into_iter().map(|(_, name)| name).collect::<Vec<_>>()
}

fn main() {
    {
        let names: Vec<String> = vec!["Mary", "John", "Emma"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let heights: Vec<i32> = vec![180, 165, 170];
        let ans: Vec<String> = vec!["Mary", "Emma", "John"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(sort_people(names, heights), ans);
    }

    {
        let names: Vec<String> = vec!["Alice", "Bob", "Bob"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let heights: Vec<i32> = vec![155, 185, 150];
        let ans: Vec<String> = vec!["Bob", "Alice", "Bob"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(sort_people(names, heights), ans);
    }
}
